#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate crypto;
extern crate codedaily_backend;
extern crate diesel;

use self::codedaily_backend::*;
use self::codedaily_backend::models::*;
use codedaily_backend::schema::links::dsl::*;
use codedaily_backend::schema::users::dsl::*;
use self::diesel::prelude::*;
use self::diesel::associations::HasTable;
use rocket_contrib::{Json, Value};
use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

const LINKS_PER_PAGE: i64 = 30;

fn encrypt_password(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(input);
    hasher.result_str()
}

#[post("/users/register", format = "application/json", data = "<user>")]
fn register_user(user: Json<User>) -> Json<Value> {
    use schema::users;

    let connection = establish_connection();
    let next_id = (users.count().get_result(&connection).unwrap_or(0) + 1) as i32;
    let new_user = User {
        id: next_id,
        username: user.username.to_string(),
        email: user.email.to_string(),
        password: encrypt_password(&user.password),
        enable: 1
    };
    let result: User = diesel::insert(&new_user).into(users::table).get_result(&connection)
                                                .expect("Error creating user");
    Json(json!({
        "result": result
    }))
}

#[get("/feed/<page>")]
fn feed(page: i64) -> Json<Value> {
    let connection = establish_connection();
    let mut offset = (page - 1) * LINKS_PER_PAGE;
    if offset < 0 {
        offset = 0;
    }
    let results = links.order(time.desc()).offset(offset).limit(LINKS_PER_PAGE).load::<Link>(&connection).ok();
    let total = links.order(time.desc()).load::<Link>(&connection).ok();
    let mut count = 0;
    if total.is_some() {
        count = total.unwrap().len();
    }
    Json(json!({
        "status": "success",
        "links": results,
        "total": count
    }))
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("www/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .mount("/api/", routes![feed, register_user])
        .mount("/", routes![index, files])
        .launch();
}
