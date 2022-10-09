use lol_html::html_content::{ContentType, Element};
use lol_html::{element, rewrite_str, RewriteStrSettings};
use rocket::fs::relative;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssr_rs::Ssr;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct SsrOutput {
    head: String,
    html: String,
}

pub fn render(path: PathBuf, ssr_content: Option<String>) -> String {
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
    let ssr_string = Ssr::render_to_string(&ssr_entry, "ssrEntry", Some(&params.to_string()));
    let ssr_output: SsrOutput = serde_json::from_str(&ssr_string).unwrap();

    rewrite_str(
        &template,
        RewriteStrSettings {
            element_content_handlers: vec![
                element!("head", |el: &mut Element| {
                    el.append(&ssr_output.head, ContentType::Html);

                    Ok(())
                }),
                element!("body", |el: &mut Element| {
                    el.append(&ssr_output.html, ContentType::Html);

                    Ok(())
                }),
            ],
            ..RewriteStrSettings::default()
        },
    )
    .unwrap()
}
