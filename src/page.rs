use super::article::{get_article_by_slug, get_articles};
use super::ssr;
use rocket::response::content;
use std::path::PathBuf;

#[get("/article/<slug>")]
pub fn article(slug: String) -> content::RawHtml<String> {
    let article = get_article_by_slug(slug.clone()).unwrap();
    let template = ssr::render(
        PathBuf::from(format!("/article/{}", slug)),
        Some(serde_json::to_string(&article).unwrap()),
    );

    content::RawHtml(template)
}

#[get("/articles")]
pub fn articles() -> content::RawHtml<String> {
    let articles = get_articles();
    let template = ssr::render(
        PathBuf::from("/articles"),
        Some(serde_json::to_string(&articles).unwrap()),
    );

    content::RawHtml(template)
}

#[get("/")]
pub fn home() -> content::RawHtml<String> {
    let article_count = 3;

    let articles = get_articles();
    let articles = &articles[..article_count];
    let template = ssr::render(
        PathBuf::from("/articles"),
        Some(serde_json::to_string(&articles).unwrap()),
    );

    content::RawHtml(template)
}
