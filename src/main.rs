use std::path::PathBuf;
#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket::response::content;

mod ymd_hm_format;
mod article;
mod page;
mod api;
mod ssr;

#[get("/<path..>")]
fn index(path: PathBuf) -> content::RawHtml<String> {
    content::RawHtml(ssr::render(path, None))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        // SSR catch-all.
        .mount("/", routes![index])
        // SSR pages.
        .mount("/", routes![page::home, page::article, page::articles])
        // API endpoints.
        .mount("/api", routes![api::article, api::articles])
        // Static files.
        .mount("/static", FileServer::from(relative!("static")).rank(-2))
}
