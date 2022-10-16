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
pub enum ProjectStatus {
    Published,
    Unpublished,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectMetadata {
    pub title: String,
    pub link: String,
    pub status: ProjectStatus,
    #[serde(with = "ymd_hm_format")]
    pub published: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub metadata: ProjectMetadata,
    pub html: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectCollection {
    pub projects: Vec<Project>,
}

impl ProjectCollection {
    pub fn new() -> Self {
        ProjectCollection {
            projects: get_projects(),
        }
    }
}

impl From<ProjectCollection> for serde_json::Value {
    fn from(value: ProjectCollection) -> Self {
        serde_json::to_value(value).unwrap()
    }
}

pub fn get_projects() -> Vec<Project> {
    let mut articles: Vec<Project> = Vec::new();

    for element in read_dir(relative!("content/projects")).unwrap() {
        let path = element.unwrap().path();
        let extension = path.extension().unwrap();
        if extension != "md" {
            continue;
        }

        articles.push(parse_project(path));
    }

    articles.sort_by(|a, b| b.metadata.published.cmp(&a.metadata.published));
    articles
}

fn parse_project(path: PathBuf) -> Project {
    let markdown = read_to_string(path).unwrap();
    let result = YamlFrontMatter::parse::<ProjectMetadata>(&markdown).unwrap();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&result.content, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);

    Project {
        metadata: result.metadata,
        html,
    }
}
