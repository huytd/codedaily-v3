#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate codedaily_backend;
extern crate diesel;
extern crate serde_json;
#[macro_use] extern crate rocket_contrib;

use self::codedaily_backend::*;
use self::codedaily_backend::models::*;
use codedaily_backend::schema::links::dsl::*;
use self::diesel::prelude::*;
use diesel::pg::PgConnection;
use rocket_contrib::{JSON, Value};
use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/feed")]
fn feed() -> JSON<Value> {
    let connection = establish_connection();
    let results = links.order(time.desc()).limit(100).load::<Link>(&connection).ok();
    JSON(json!({
        "status": "success",
        "message": results
    }))
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![files])
        .mount("/api/", routes![feed])
        .launch();
}
