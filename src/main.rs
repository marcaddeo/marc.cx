#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket::response::content;
use ssr_rs::Ssr;
use std::fs::read_to_string;
use std::path::PathBuf;
use lol_html::{rewrite_str, element, RewriteStrSettings};
use lol_html::html_content::{ContentType, Element};
use serde_json::json;

fn ssr(path: PathBuf, ssr_content: Option<String>) -> String {
    let ssr_content_string = match ssr_content {
        Some(content) => content,
        None => String::from("null"),
    };

    let params = json!({
        "url": &path,
        "ssrContent": ssr_content_string,
    });

    let template = read_to_string(relative!("static/index.html")).unwrap();
    let ssr_entry = read_to_string(relative!("static/build/ssrEntry.js")).unwrap();
    let inner_html = Ssr::render_to_string(&ssr_entry, "ssrEntry", Some(&params.to_string()));

    rewrite_str(
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
    ).unwrap()
}

#[get("/article")]
fn article() -> content::RawHtml<String> {
    let article = read_to_string(relative!("articles/test.txt")).unwrap();
    content::RawHtml(article)
}

#[get("/article/<slug>")]
fn article_test(slug: String) -> content::RawHtml<String> {
    let article = read_to_string(relative!("articles/test.txt")).unwrap();
    let template = ssr(PathBuf::from(format!("/article/{}", slug)), Some(article));

    content::RawHtml(template)
}

#[get("/<path..>", rank = 11)]
fn index(path: PathBuf) -> content::RawHtml<String> {
    content::RawHtml(ssr(path, None))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, article_test])
        .mount("/api", routes![article])
        .mount("/static", FileServer::from(relative!("static")))
}
