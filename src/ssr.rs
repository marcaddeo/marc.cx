use lol_html::html_content::{ContentType, Element};
use lol_html::{element, rewrite_str, RewriteStrSettings};
use rocket::fs::relative;
use serde::{Deserialize, Serialize};
use ssr_rs::Ssr;
use std::fs::read_to_string;
use rocket::http::uri::Origin;

#[derive(Serialize, Deserialize, Debug)]
struct SsrOutput {
    head: String,
    html: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SsrInput<'a, J>
where
    J: Into<serde_json::Value> + Serialize,
{
    url: Origin<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    props: Option<J>,
}

pub fn render<J>(path: Origin, props: Option<J>) -> String
where
    J: Into<serde_json::Value> + Serialize,
{
    let params = SsrInput { url: path, props };

    let template = read_to_string(relative!("static/index.html")).unwrap();
    let ssr_entry = read_to_string(relative!("static/build/ssrEntry.js")).unwrap();
    let ssr_input = serde_json::to_string(&params).unwrap();
    let ssr_string = Ssr::render_to_string(&ssr_entry, "ssrEntry", Some(&ssr_input));
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
