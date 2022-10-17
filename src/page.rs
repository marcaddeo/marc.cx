use super::article::{ArticleCollection, ArticleEntry, SpecificArticle};
use super::project::ProjectCollection;
use super::api;
use super::ssr;
use rocket::http::Status;
use rocket::response::content;

#[get("/article/<slug>")]
pub fn article(slug: String) -> (Status, content::RawHtml<String>) {
    let article: SpecificArticle = api::article(slug.clone()).into();
    let status = match article.article {
        ArticleEntry::Article(_) => Status::Ok,
        ArticleEntry::NotFound { .. } => Status::NotFound,
    };

    let html = ssr::render(uri!(article(slug)), Some(article));

    (status, content::RawHtml(html))
}

#[get("/articles")]
pub fn articles() -> content::RawHtml<String> {
    let collection: ArticleCollection = api::articles(None, None).into();
    let html = ssr::render(uri!(articles()), Some(collection));

    content::RawHtml(html)
}

#[get("/article/tag/<tag>")]
pub fn tag(tag: String) -> (Status, content::RawHtml<String>) {
    let collection: ArticleCollection = api::articles(None, Some(tag.clone())).into();
    let html = ssr::render(uri!(tag(tag)), Some(collection.clone()));
    let status = match collection.articles.len() {
        0 => Status::NotFound,
        _ => Status::Ok,
    };

    (status, content::RawHtml(html))
}

#[get("/projects")]
pub fn projects() -> content::RawHtml<String> {
    let projects: ProjectCollection = api::projects().into();
    let html = ssr::render(uri!(projects()), Some(projects));

    content::RawHtml(html)
}

#[get("/")]
pub fn home() -> content::RawHtml<String> {
    let collection: ArticleCollection = api::articles(Some(3usize), None).into();
    let html = ssr::render(uri!(home()), Some(collection));

    content::RawHtml(html)
}
