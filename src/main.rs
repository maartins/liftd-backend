use rocket::*;
use rocket::fs::NamedFile;

use std::path::{Path, PathBuf};

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("index.html")).await.ok()
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[catch(404)]
async fn not_found() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("index.html")).await.ok()
}


#[launch]
fn rocket() -> _ {
    rocket::build().register("/", catchers![not_found]).mount("/", routes![index, files])
}