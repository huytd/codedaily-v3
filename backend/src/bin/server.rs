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

#[get("/")]
fn index() -> JSON<Value> {
    JSON(json!({
        "status": "success",
        "message": "It's worked!"
    }))
}

#[get("/feed")]
fn feed() -> JSON<Value> {
    let connection = establish_connection();
    let results = links.order(time.desc()).limit(100).load::<Link>(&connection).ok();
    JSON(json!({
        "status": "success",
        "message": results
    }))
}

fn main() {
    rocket::ignite().mount("/", routes![index, feed]).launch();
}
