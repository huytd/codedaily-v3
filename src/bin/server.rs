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

    let found_exist_user: i32 = users.count()
                                .filter(users::username.eq(&new_user.username).or(users::email.eq(&new_user.email)))
                                .get_result(&connection).unwrap_or(0) as i32;

    if found_exist_user <= 0 {
        let result: User = diesel::insert(&new_user).into(users::table).get_result(&connection)
            .expect("Error creating user");
        Json(json!({
            "result": result
        }))
    } else {
        Json(json!({
            "result": false
        }))
    }
}

#[post("/users/login", format = "application/json", data="<user>")]
fn login_user(user: Json<Value>) -> Json<Value> {
    use schema::users;

    let connection = establish_connection();

    let t_username = user["username"].as_str().unwrap_or("");
    let t_password = encrypt_password(user["password"].as_str().unwrap_or(""));

    let found_user = users.filter(username.eq(t_username).and(password.eq(t_password)))
                          .load::<User>(&connection).ok();
    Json(json!({
        "result": found_user
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

#[get("/<file..>", rank = 5)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .mount("/api/", routes![feed, register_user, login_user])
        .mount("/", routes![index, files])
        .launch();
}
