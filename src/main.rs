#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

// Route for specific file paths
#[get("/<file..>", rank = 2)]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

// Default route when no specific path is provided, serves index.html
#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, files])
}