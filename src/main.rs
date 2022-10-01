#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
use rocket::http::ContentType;

#[get("/global.css")]
fn global_css() -> (ContentType, String) {
    let sass = grass::from_path(
        relative!("static/global.css"),
        &grass::Options::default()
    ).unwrap();

    (ContentType::CSS, sass)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![global_css])
        .mount("/", FileServer::from(relative!("static")))
}
