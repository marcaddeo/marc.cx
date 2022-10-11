use super::article::{SpecificArticle, ArticleEntry, ArticleCollection};
use super::ssr;
use rocket::response::content;
use rocket::http::Status;

#[get("/article/<slug>")]
pub fn article(slug: String) -> (Status, content::RawHtml<String>) {
    let article = SpecificArticle::new(slug.clone());
    let status = match article.article {
        ArticleEntry::Article(_) => Status::Ok,
        ArticleEntry::NotFound { .. } => Status::NotFound,
    };

    let html = ssr::render(
        format!("/article/{}", slug),
        Some(article)
    );

    (status, content::RawHtml(html))
}

#[get("/articles")]
pub fn articles() -> content::RawHtml<String> {
    let articles = ArticleCollection::new();
    let html = ssr::render(
        "/articles",
        Some(articles),
    );

    content::RawHtml(html)
}

#[get("/")]
pub fn home() -> content::RawHtml<String> {
    let articles = ArticleCollection::new_ext(3);
    let html = ssr::render(
        "/",
        Some(serde_json::to_value(articles).unwrap()),
    );

    content::RawHtml(html)
}
