use rocket::serde::json::Json;
use super::article::{Article, get_articles, get_article_by_slug};

#[get("/article/<slug>", format = "json")]
pub fn article(slug: String) -> Option<Json<Article>> {
    match get_article_by_slug(slug) {
        Some(article) => Some(Json(article)),
        None => None
    }
}

#[get("/articles?<limit>", format = "json")]
pub fn articles(limit: Option<usize>) -> Json<Vec<Article>> {
    let mut articles = get_articles();
    articles.sort_by(|a, b| b.metadata.published.cmp(&a.metadata.published));

    match limit {
        Some(limit) => Json(articles[..limit].to_vec()),
        None => Json(articles)
    }
}
