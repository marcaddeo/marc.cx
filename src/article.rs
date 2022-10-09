use super::ymd_hm_format;
use chrono::{DateTime, Utc};
use pulldown_cmark::{html, Options, Parser};
use rocket::fs::relative;
use serde::{Deserialize, Serialize};
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use yaml_front_matter::YamlFrontMatter;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ArticleStatus {
    Published,
    Unpublished,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArticleMetadata {
    pub title: String,
    pub slug: String,
    pub status: ArticleStatus,
    #[serde(with = "ymd_hm_format")]
    pub published: DateTime<Utc>,
    pub excerpt: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Article {
    pub metadata: ArticleMetadata,
    pub html: String,
}

pub fn get_articles() -> Vec<Article> {
    let mut articles: Vec<Article> = Vec::new();

    for element in read_dir(relative!("content/articles")).unwrap() {
        let path = element.unwrap().path();
        let extension = path.extension().unwrap();
        if extension != "md" {
            continue;
        }

        articles.push(parse_article(path));
    }

    articles
}

pub fn get_article_by_slug(slug: String) -> Option<Article> {
    let articles = get_articles();

    for article in articles {
        if article.metadata.slug == slug {
            return Some(article);
        }
    }

    None
}
fn parse_article(path: PathBuf) -> Article {
    let markdown = read_to_string(path).unwrap();
    let result = YamlFrontMatter::parse::<ArticleMetadata>(&markdown).unwrap();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&result.content, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);

    Article {
        metadata: result.metadata,
        html,
    }
}
