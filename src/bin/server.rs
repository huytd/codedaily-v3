#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate rocket_contrib;
extern crate crypto;
extern crate codedaily_backend;
extern crate diesel;

use self::codedaily_backend::*;
use self::codedaily_backend::models::*;
use self::codedaily_backend::middleware::Auth;
use self::codedaily_backend::helpers::*;
use codedaily_backend::schema::links::dsl::*;
use codedaily_backend::schema::users::dsl::*;
use codedaily_backend::schema::comments::dsl::*;
use self::diesel::prelude::*;
use rocket_contrib::{Json, Value};
use std::io;
use std::cmp::max;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

const LINKS_PER_PAGE: i64 = 30;
const COMMENTS_PER_PAGE: u32 = 30;

fn encrypt_password(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(input);
    hasher.result_str()
}

#[post("/users/register", format = "application/json", data = "<user>")]
fn register_user(user: Json<User>) -> Json<Value> {
    use schema::users;

    let connection = establish_connection();

    let new_user = NewUser {
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
    use schema::auth_tokens;

    let connection = establish_connection();

    let t_username = user["username"].as_str().unwrap_or("");
    let t_password = encrypt_password(user["password"].as_str().unwrap_or(""));

    let result = User::find_by_login(&connection, t_username, t_password.as_str());

    match result {
        Ok(user) => {
            let rand_hash = gen_random_hash();
            let expired_at = (epoch_now() as i64) + AUTH_TOKEN_TTL;
            let new_auth_token = AuthToken {
                token: rand_hash,
                expired_at: expired_at,
                user_id: user.id,
            };
            let result: AuthToken = diesel::insert(&new_auth_token).into(auth_tokens::table).get_result(&connection)
                .expect("Error creating auth token");

            Json(json!({
                "result": true,
                "user": {
                    "username": user.username,
                    "email": user.email,
                },
                "token": result.token,
            }))
        },
        Err(_) => {
            Json(json!({
                "result": false,
            }))
        }
    }
}

#[get("/users/me")]
fn get_user(auth: Auth) -> Json<Value> {
    let connection = establish_connection();

    let user = User::find(&connection, auth.user_id).unwrap();

    Json(json!({
        "result": true,
        "user": {
            "username": user.username,
            "email": user.email,
        },
        "token": auth.token,
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

#[get("/link/<link>/comments")]
fn get_comments(link: i32) -> Json<Value> {
    get_comments_page(link, 1)
}

#[get("/link/<link>/comments/<page>")]
fn get_comments_page(link: i32, page: u32) -> Json<Value> {
    let connection = establish_connection();
    let offset = max(page - 1, 0) * COMMENTS_PER_PAGE;

    let comments_query = comments.filter(link_id.eq(link));
    let results = comments_query.offset(offset as i64).limit(LINKS_PER_PAGE).load::<Comment>(&connection).ok();
    let total = comments_query.load::<Comment>(&connection).ok();

    Json(json!({
        "status": "success",
        "comments": results,
        "total": total.unwrap().len()
    }))
}

#[post("/link/<link>/comment", format = "application/json", data = "<comment>")]
fn post_comment(auth: Auth, link: i32, comment: Json<PostComment>) -> Json<Value> {
    use schema::comments;

    let connection = establish_connection();

    let new_comment = NewComment {
        message: comment.message.to_string(),
        link_id: link,
        author_id: auth.user_id,
    };

    let result: Comment = diesel::insert(&new_comment).into(comments::table).get_result(&connection)
        .expect("Error persisting comment");

    Json(json!({
        "id": result.id
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
        .mount("/api/", routes![feed, register_user, login_user, get_user, get_comments, get_comments_page, post_comment])
        .mount("/", routes![index, files])
        .launch();
}
