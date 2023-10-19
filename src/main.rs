#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;
use std::path::{Path};

#[get("/")]
async fn home() -> Option<NamedFile>{
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![home])
}