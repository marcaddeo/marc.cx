use super::article::{get_article_by_slug, get_articles, Article};
use super::project::{get_projects, Project};
use rocket::serde::json::Json;

#[get("/article/<slug>", format = "json")]
pub fn article(slug: String) -> Option<Json<Article>> {
    match get_article_by_slug(slug) {
        Some(article) => Some(Json(article)),
        None => None,
    }
}

#[get("/articles?<limit>", format = "json")]
pub fn articles(limit: Option<usize>) -> Json<Vec<Article>> {
    match limit {
        Some(limit) => Json(get_articles()[..limit].to_vec()),
        None => Json(get_articles()),
    }
}

#[get("/projects", format = "json")]
pub fn projects() -> Json<Vec<Project>> {
    Json(get_projects())
}
