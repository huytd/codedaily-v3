extern crate codedaily_backend;
extern crate diesel;
extern crate url;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;
extern crate feed_parser;

use self::codedaily_backend::*;
use self::codedaily_backend::models::*;
use self::diesel::prelude::*;
use diesel::pg::PgConnection;
use codedaily_backend::schema::sites::dsl::*;
use codedaily_backend::schema::links::dsl::*;
use url::{Url};
use std::thread;
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;
use feed_parser::parser;

fn insert_link(conn: &PgConnection, post_url: &str, post_title: &str, post_time: i32) -> Link {
    use schema::links;

    let parsed_url = Url::parse(post_url).unwrap();
    let new_link = NewLink {
        title: post_title.to_string(),
        url: post_url.to_string(),
        time: post_time,
        body: Some("".to_string()),
        source: Some(parsed_url.host_str().unwrap_or("").to_string()),
    };
    diesel::insert(&new_link).into(links::table)
        .get_result(conn)
        .expect("Error saving new link")
}

fn crawl_site(connection: &PgConnection, site: &Site) -> i32 {
    let feed = parser::from_url(&site.url);
    match feed {
        Some(feed) => {
            let mut latestcheck = site.last_check;
            for entry in feed.entries {
                let post_title = &entry.title.unwrap_or("".to_string());
                let post_url = &entry.id;
                let pub_date = entry.published;
                let post_time = pub_date.timestamp() as i32;
                println!("Checking {} {}", post_time, latestcheck);
                if post_time > latestcheck {
                    latestcheck = post_time;
                }
                if post_time > site.last_check {
                    println!("{} : {} : {}", post_title, post_url, post_time);
                    insert_link(&connection, post_url, post_title, post_time);
                }
            }
            latestcheck
        },
        None => -1
    }
}

fn save_last_check(connection: &PgConnection, site: &Site, last_check_time: i32) -> Site {
    diesel::update(sites.find(site.id))
        .set(last_check.eq(last_check_time))
        .get_result(connection)
        .expect("Unable to update site")
}

fn main() {
    let connection = establish_connection();
    let results = sites.load::<Site>(&connection).expect("Error loading sites");

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let config = r2d2::Config::default();
    let pool = r2d2::Pool::new(config, manager).expect("Failed to create pool.");

    let mut thread_pool = vec![];

    for site in results {
        let pool = pool.clone();

        thread_pool.push(thread::spawn(move || {
            let connection = pool.get().ok().expect("Cannot get connection pool");
            let last_check_time = crawl_site(&connection, &site);
            if last_check_time != -1 {
                save_last_check(&connection, &site, last_check_time);
            }
        }));
    }

    for t in thread_pool {
        let _ = t.join();
    }
}
