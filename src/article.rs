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

fn parse_article(path: PathBuf) -> Article {
    let markdown = read_to_string(path).unwrap();
    let document = YamlFrontMatter::parse::<ArticleMetadata>(&markdown).unwrap();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(&document.content, options);
    let mut html = String::new();
    markdown::push_html(&mut html, parser);

    Article {
        metadata: document.metadata,
        html,
    }
}
