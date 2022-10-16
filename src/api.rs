use super::article::{Article, ArticleEntry, ArticleCollectionBuilder, SpecificArticle};
use super::project::{Project, ProjectCollection};
use rocket::serde::json::Json;

#[get("/article/<slug>", format = "json")]
pub fn article(slug: String) -> Option<Json<Article>> {
    let article = SpecificArticle::new(slug);

    match article.article {
        ArticleEntry::Article(article) => Some(Json(article)),
        ArticleEntry::NotFound { .. } => None,
    }
}

#[get("/articles?<limit>&<tag>", format = "json")]
pub fn articles(limit: Option<usize>, tag: Option<String>) -> Json<Vec<Article>> {
    let collection = ArticleCollectionBuilder::default()
        .tag(tag)
        .limit(limit)
        .build()
        .unwrap();

    Json(collection.articles)
}

#[get("/projects", format = "json")]
pub fn projects() -> Json<Vec<Project>> {
    let collection = ProjectCollection::new();

    Json(collection.projects)
}
