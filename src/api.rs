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

#[get("/articles?<limit>&<tag>", format = "json")]
pub fn articles(limit: Option<usize>, tag: Option<String>) -> Json<Vec<Article>> {
    let mut articles = get_articles();

    if let Some(tag) = tag {
        articles = articles
            .into_iter()
            .filter(|article| article.metadata.tags.contains(&tag))
            .collect();
    }

    if let Some(limit) = limit {
        if articles.len() > limit {
            articles = articles[..limit].to_vec();
        }
    }

    Json(articles)
}

#[get("/projects", format = "json")]
pub fn projects() -> Json<Vec<Project>> {
    Json(get_projects())
}
