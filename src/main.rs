#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;
use rocket::Route;
use std::path::{Path, PathBuf};

// #[get("/")]
// fn hello() -> &'static str {
//     "Hello, world!"
// }

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile>{
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![files])
}