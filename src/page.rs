use super::article::{ArticleCollectionBuilder, ArticleEntry, SpecificArticle};
use super::project::ProjectCollection;
use super::ssr;
use rocket::http::Status;
use rocket::response::content;

#[get("/article/<slug>")]
pub fn article(slug: String) -> (Status, content::RawHtml<String>) {
    let article = SpecificArticle::new(slug.clone());
    let status = match article.article {
        ArticleEntry::Article(_) => Status::Ok,
        ArticleEntry::NotFound { .. } => Status::NotFound,
    };

    let html = ssr::render(format!("/article/{}", slug), Some(article));

    (status, content::RawHtml(html))
}

#[get("/articles")]
pub fn articles() -> content::RawHtml<String> {
    let articles = ArticleCollectionBuilder::default().build().unwrap();
    let html = ssr::render("/articles", Some(articles));

    content::RawHtml(html)
}

#[get("/article/tag/<tag>")]
pub fn tag(tag: String) -> (Status, content::RawHtml<String>) {
    let articles = ArticleCollectionBuilder::default()
        .tag(tag.clone())
        .build()
        .unwrap();
    let html = ssr::render(format!("/article/tag/{}", tag), Some(articles.clone()));

    let status = match articles.articles.len() {
        0 => Status::NotFound,
        _ => Status::Ok,
    };

    (status, content::RawHtml(html))
}

#[get("/projects")]
pub fn projects() -> content::RawHtml<String> {
    let html = ssr::render("/projects", Some(ProjectCollection::new()));

    content::RawHtml(html)
}

#[get("/")]
pub fn home() -> content::RawHtml<String> {
    let articles = ArticleCollectionBuilder::default()
        .limit(3usize)
        .build()
        .unwrap();
    let html = ssr::render("/", Some(articles));

    content::RawHtml(html)
}
