use super::article::{ArticleCollection, ArticleEntry, SpecificArticle};
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
    let html = ssr::render("/articles", Some(ArticleCollection::new()));

    content::RawHtml(html)
}

#[get("/projects")]
pub fn projects() -> content::RawHtml<String> {
    let html = ssr::render("/projects", Some(ProjectCollection::new()));

    content::RawHtml(html)
}

#[get("/")]
pub fn home() -> content::RawHtml<String> {
    let html = ssr::render("/", Some(ArticleCollection::new_ext(3)));

    content::RawHtml(html)
}
