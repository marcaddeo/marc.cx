use std::path::PathBuf;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate derive_builder;
use rocket::fs::{relative, FileServer};
use rocket::http::Status;
use rocket::response::content;

mod api;
mod article;
mod page;
mod project;
mod ssr;
mod ymd_hm_format;

#[get("/<path..>")]
fn index(path: PathBuf) -> (Status, content::RawHtml<String>) {
    let html = ssr::render(uri!(index(path)), None::<serde_json::Value>);
    let status = if html.contains("window.not_found = true;") {
        Status::NotFound
    } else {
        Status::Ok
    };

    (status, content::RawHtml(html))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        // SSR catch-all.
        .mount("/", routes![index])
        // SSR pages.
        .mount(
            "/",
            routes![
                page::home,
                page::article,
                page::articles,
                page::tag,
                page::projects
            ],
        )
        // API endpoints.
        .mount("/api", routes![api::article, api::articles, api::projects])
        // Article assets.
        .mount(
            "/article",
            FileServer::from(relative!("content/articles")).rank(-2),
        )
        // Static files.
        .mount("/static", FileServer::from(relative!("static")).rank(-2))
}
