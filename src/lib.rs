#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate error_chain;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
extern crate url;
#[macro_use] extern crate serde_derive;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;
pub mod errors;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").chain_err("DATABASE_URL must be set");
    PgConnection::establish(&database_url).chain_err(&format!("Error connecting to {}", database_url))
}
