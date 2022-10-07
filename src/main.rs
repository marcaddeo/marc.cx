#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket::response::content;
use ssr_rs::Ssr;
use std::fs::read_to_string;
use std::path::PathBuf;
use lol_html::{rewrite_str, element, RewriteStrSettings};
use lol_html::html_content::{ContentType, Element};

#[get("/<path..>", rank = 11)]
fn index(path: PathBuf) -> content::RawHtml<String> {
    let template = read_to_string(relative!("static/index.html")).unwrap();
    let ssr_entry = read_to_string(relative!("static/build/ssrEntry.js")).unwrap();
    let inner_html = Ssr::render_to_string(&ssr_entry, "ssrEntry", Some(path.to_str().unwrap_or("/")));
    let html = rewrite_str(
        &template,
        RewriteStrSettings {
            element_content_handlers: vec![
                element!("body", |el: &mut Element| {
                    el.append(&inner_html, ContentType::Html);

                    Ok(())
                }),
            ],
            ..RewriteStrSettings::default()
        }
    ).unwrap();

    content::RawHtml(html)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("static")))
}
