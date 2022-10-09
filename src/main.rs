#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket::response::content;
use ssr_rs::Ssr;
use std::fs::{read_to_string, read_dir};
use std::path::PathBuf;
use lol_html::{rewrite_str, element, RewriteStrSettings};
use lol_html::html_content::{ContentType, Element};
use serde::{Serialize, Deserialize};
use serde_json::json;
use yaml_front_matter::YamlFrontMatter;
use pulldown_cmark::{Parser, Options, html};
use rocket::serde::json::Json;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
struct SsrOutput {
    head: String,
    html: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
enum ArticleStatus {
    Published,
    Unpublished,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ArticleMetadata {
    title: String,
    slug: String,
    status: ArticleStatus,
    #[serde(with = "ymd_hm_format")]
    published: DateTime<Utc>,
    excerpt: String,
    tags: Vec<String>,
}

#[derive(Serialize, Debug, Clone)]
struct Article {
    metadata: ArticleMetadata,
    html: String,
}

mod ymd_hm_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M";


    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

fn parse_article(path: PathBuf) -> Article {
    let markdown = read_to_string(path).unwrap();
    let result = YamlFrontMatter::parse::<ArticleMetadata>(&markdown).unwrap();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&result.content, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);

    Article {
        metadata: result.metadata,
        html,
    }
}

fn get_articles() -> Vec<Article> {
    let mut articles: Vec<Article> = Vec::new();

    for element in read_dir(relative!("content/articles")).unwrap() {
        let path = element.unwrap().path();
        let extension = path.extension().unwrap();
        if extension != "md" {
            continue;
        }

        articles.push(parse_article(path));
    }

    articles
}

fn get_article_by_slug(slug: String) -> Option<Article> {
    let articles = get_articles();

    for article in articles {
        if article.metadata.slug == slug {
            return Some(article)
        }
    }

    None
}

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
        }
    ).unwrap()
}

#[get("/article/<slug>", format = "json")]
fn api_article(slug: String) -> Option<Json<Article>> {
    match get_article_by_slug(slug) {
        Some(article) => Some(Json(article)),
        None => None
    }
}

#[get("/articles?<limit>", format = "json")]
fn api_articles(limit: Option<usize>) -> Json<Vec<Article>> {
    let mut articles = get_articles();
    articles.sort_by(|a, b| b.metadata.published.cmp(&a.metadata.published));

    match limit {
        Some(limit) => Json(articles[..limit].to_vec()),
        None => Json(articles)
    }
}

#[get("/article/<slug>")]
fn article(slug: String) -> content::RawHtml<String> {
    let article = get_article_by_slug(slug.clone()).unwrap();
    let template = ssr(PathBuf::from(format!("/article/{}", slug)), Some(serde_json::to_string(&article).unwrap()));

    content::RawHtml(template)
}

#[get("/articles")]
fn articles() -> content::RawHtml<String> {
    let articles = get_articles();
    let template = ssr(PathBuf::from("/articles"), Some(serde_json::to_string(&articles).unwrap()));

    content::RawHtml(template)
}

#[get("/<path..>")]
fn index(path: PathBuf) -> content::RawHtml<String> {
    content::RawHtml(ssr(path, None))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, article, articles])
        .mount("/api", routes![api_article, api_articles])
        .mount("/static", FileServer::from(relative!("static")).rank(-2))
}
