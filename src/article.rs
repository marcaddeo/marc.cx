use super::markdown;
use super::ymd_hm_format;
use chrono::{DateTime, Utc};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use rocket::fs::relative;
use rocket::serde::json::Json;
use serde::ser::{SerializeStruct, SerializeStructVariant};
use serde::{Deserialize, Serialize};
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use yaml_front_matter::YamlFrontMatter;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ArticleStatus {
    Published,
    Unpublished,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArticleMetadata {
    pub title: String,
    pub clean_title: Option<String>,
    pub slug: String,
    pub status: ArticleStatus,
    #[serde(with = "ymd_hm_format")]
    pub published: DateTime<Utc>,
    pub excerpt: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    pub metadata: ArticleMetadata,
    pub html: String,
}

#[derive(Builder, Serialize, Deserialize, Debug, Clone)]
#[builder(build_fn(private, name = "internal_build"))]
pub struct ArticleCollection {
    #[builder(setter(skip))]
    #[builder(default = "self.default_articles()")]
    pub articles: Vec<Article>,
    #[serde(skip)]
    #[builder(setter(into), default)]
    #[allow(dead_code)]
    limit: Option<usize>,
    #[serde(skip)]
    #[builder(setter(into), default)]
    #[allow(dead_code)]
    tag: Option<String>,
}

impl ArticleCollectionBuilder {
    fn default_articles(&self) -> Vec<Article> {
        get_articles()
    }

    pub fn build(&self) -> Result<ArticleCollection, ArticleCollectionBuilderError> {
        let mut collection = self.internal_build()?;

        if let Some(Some(tag)) = &self.tag {
            collection
                .articles
                .retain(|article| article.metadata.tags.contains(tag));
        }

        if let Some(Some(limit)) = self.limit {
            if collection.articles.len() > limit {
                collection.articles = collection.articles[..limit].to_vec();
            }
        }

        Ok(collection)
    }
}

impl From<Json<Vec<Article>>> for ArticleCollection {
    fn from(articles: Json<Vec<Article>>) -> Self {
        Self {
            articles: articles.into_inner(),
            limit: None,
            tag: None,
        }
    }
}

impl From<ArticleCollection> for serde_json::Value {
    fn from(value: ArticleCollection) -> Self {
        serde_json::to_value(value).unwrap()
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ArticleEntry {
    Article(Article),
    NotFound {
        code: u16,
        description: String,
        reason: String,
    },
}

impl serde::ser::Serialize for ArticleEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ArticleEntry::Article(article) => {
                let mut ss = serializer.serialize_struct("Article", 2)?;
                ss.serialize_field("metadata", &article.metadata)?;
                ss.serialize_field("html", &article.html)?;
                ss.end()
            }
            ArticleEntry::NotFound {
                code,
                description,
                reason,
            } => {
                let mut sv = serializer.serialize_struct_variant("ArticleEntry", 0, "error", 3)?;
                sv.serialize_field("code", code)?;
                sv.serialize_field("description", description)?;
                sv.serialize_field("reason", reason)?;
                sv.end()
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecificArticle {
    pub article: ArticleEntry,
}

impl SpecificArticle {
    pub fn new(slug: String) -> Self {
        let article = get_article_by_slug(slug);

        match article {
            Some(article) => Self {
                article: ArticleEntry::Article(article),
            },
            None => Self {
                article: ArticleEntry::NotFound {
                    code: 404,
                    description: String::from("The requested resource could not be found."),
                    reason: String::from("Not Found"),
                },
            },
        }
    }
}

impl From<SpecificArticle> for serde_json::Value {
    fn from(value: SpecificArticle) -> Self {
        serde_json::to_value(value).unwrap()
    }
}

impl From<Option<Json<Article>>> for SpecificArticle {
    fn from(article: Option<Json<Article>>) -> Self {
        match article {
            Some(article) => Self {
                article: ArticleEntry::Article(article.into_inner()),
            },
            None => Self {
                article: ArticleEntry::NotFound {
                    code: 404,
                    description: String::from("The requested resource could not be found."),
                    reason: String::from("Not Found"),
                },
            },
        }
    }
}

pub fn get_articles() -> Vec<Article> {
    let mut articles: Vec<Article> = Vec::new();

    for element in read_dir(relative!("content/articles")).unwrap() {
        let path = element.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension != "md" {
                continue;
            }
        } else {
            continue;
        }

        articles.push(parse_article(path));
    }

    let environment = match std::env::var("APP_ENV") {
        Ok(val) => val,
        Err(_) => "production".to_string(),
    };

    // Filter out unpublished articles on Production.
    if environment == "production" {
        articles = articles
            .into_iter()
            .filter(|article| article.metadata.status == ArticleStatus::Published)
            .collect();
    }
    articles.sort_by(|a, b| b.metadata.published.cmp(&a.metadata.published));
    articles
}

pub fn get_article_by_slug(slug: String) -> Option<Article> {
    get_articles()
        .into_iter()
        .find(|article| article.metadata.slug == slug)
}

macro_rules! markdown_feature {
    ($event:ident, $($tt:tt)*) => {
        let $event = if $event.is_some() {
            match $event.unwrap() {
                $($tt)*
                event => Some(event),
            }
        } else {
            None
        };
    };
}

fn parse_article(path: PathBuf) -> Article {
    let markdown = read_to_string(path).unwrap();
    let document = YamlFrontMatter::parse::<ArticleMetadata>(&markdown).unwrap();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    // Code block handling.
    let mut code: String = String::new();
    let mut in_code_block = false;
    let mut syntax: String = String::new();

    // Link handling.
    let mut in_link = false;
    let mut link_text: String = String::new();

    // Blockquote handling.
    let mut in_blockquote = false;
    let mut blockquote_text: String = String::new();

    let parser = Parser::new_ext(&document.content, options).filter_map(|event| {
        let event = Some(event);

        // Turn code blocks into <code-block> web components with <noscript> support.
        markdown_feature! {
            event,

            Event::Start(Tag::CodeBlock(info)) => {
                in_code_block = true;
                code = String::new();

                syntax = match info {
                    CodeBlockKind::Fenced(ref language) => language.to_string(),
                    _ => String::from("plaintext"),
                };

                None
            }
            Event::Text(text) => {
                if in_code_block {
                    code += &text;

                    None
                } else {
                    Some(Event::Text(text))
                }
            }
            Event::End(Tag::CodeBlock(_)) => {
                in_code_block = false;

                code = html_escape::encode_safe(&code).into();
                let html = format!(
                    r##"
                    <noscript><pre><code>{}</code></pre></noscript>
                    <code-block language="{}" code="{}"></code-block>
                    "##,
                    code, syntax, code
                );
                Some(Event::Html(html.into()))
            }
        }

        // Handle external links.
        markdown_feature! {
            event,

            Event::Start(Tag::Link(_, _, _)) => {
                in_link = true;
                link_text = String::new();

                None
            }
            Event::Text(text) => {
                if in_link {
                    link_text += &text;

                    None
                } else {
                    Some(Event::Text(text))
                }
            }
            Event::End(Tag::Link(_, destination, link_title)) => {
                in_link = false;

                let is_external = !(destination.starts_with("/")
                    || destination.starts_with("https://marc.cx"));

                let mut title = String::new();
                if !link_title.is_empty() {
                    title = format!(" title=\"{}\" ", link_title);
                }
                let mut external_attributes = "";
                if is_external {
                    external_attributes = r##"target="_blank" rel="noopener" class="external""##
                }
                let html = format!(
                    r##"
                    <a href="{}"{}{}>{}</a>
                    "##,
                    destination,
                    title,
                    external_attributes,
                    link_text.trim()
                );

                Some(Event::Html(html.into()))
            }
        }

        // GitHub-style blockquote alerts.
        markdown_feature! {
            event,

            Event::Start(Tag::BlockQuote) => {
                in_blockquote = true;
                blockquote_text = String::new();

                None
            }
            Event::SoftBreak => {
                blockquote_text += "\n";

                None
            }
            Event::Start(Tag::Paragraph) => {
                blockquote_text += "<p>";

                None
            }
            Event::End(Tag::Paragraph) => {
                blockquote_text += "</p>";

                None
            }
            Event::Text(text) => {
                if in_blockquote {
                    blockquote_text += &text;

                    None
                } else {
                    Some(Event::Text(text))
                }
            }
            Event::End(Tag::BlockQuote) => {
                in_blockquote = false;

                let mut lines = blockquote_text.lines();
                let first_line = lines.next();

                let mut alert_type = String::new();
                let mut quote = String::new();

                if let Some(mut line) = first_line {
                    if line.starts_with("<p>") {
                        line = line.strip_prefix("<p>").unwrap();
                        quote += "<p>";
                    }
                    match line {
                        "[!NOTE]" => {
                            alert_type = "note".into();
                        }
                        "[!IMPORTANT]" => {
                            alert_type = "important".into();
                        }
                        "[!WARNING]" => {
                            alert_type = "warning".into();
                        }
                        _ => (),
                    }

                    // If this isn't an alert, we need to preserve the first
                    // line.
                    if alert_type.is_empty() {
                        quote += line;
                    }
                }

                // Join the remaining lines back together.
                quote += &lines.collect::<Vec<_>>().join("\n");

                let mut html = format!("<blockquote>{}</blockquote>", quote);
                if !alert_type.is_empty() {
                    html = format!(
                        r##"
                        <noscript><blockquote class="alert--{}">{}</blockquote></noscript>
                        <alert-block type="{}" value="{}"></alert-block>
                        "##,
                        alert_type,
                        quote,
                        alert_type,
                        quote,
                    );
                }

                Some(Event::Html(html.into()))
            }
        }

        event
    });

    let mut html = String::new();
    markdown::push_html(&mut html, parser);

    Article {
        metadata: document.metadata,
        html,
    }
}
