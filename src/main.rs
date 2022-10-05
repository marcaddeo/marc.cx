#[macro_use] extern crate rocket;
use std::path::PathBuf;
use rocket::fs::{FileServer, relative};
use rocket::response::content;
use ssr_rs::Ssr;
use std::fs::read_to_string;

#[get("/<path..>", rank = 11)]
fn index(path: PathBuf) -> content::RawHtml<String> {
    let source = read_to_string(relative!("static/build/ssrEntry.js")).unwrap();
    let html = Ssr::render_to_string(&source, "SSR", Some(path.to_str().unwrap_or("/")));

    content::RawHtml(html)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("static")))
}
