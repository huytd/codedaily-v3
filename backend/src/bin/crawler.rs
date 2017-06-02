extern crate codedaily_backend;
extern crate diesel;
extern crate rss;
extern crate chrono;
extern crate url;

use self::codedaily_backend::*;
use self::codedaily_backend::models::*;
use self::diesel::prelude::*;
use diesel::pg::PgConnection;
use rss::Channel;
use chrono::prelude::*;
use codedaily_backend::schema::sites::dsl::*;
use codedaily_backend::schema::links::dsl::*;
use url::{Url, Host};

fn insert_link(conn: &PgConnection, post_url: &str, post_title: &str, post_time: i32) -> Link {
    use schema::links;

    let parsed_url = Url::parse(post_url).unwrap();
    let new_link = NewLink {
        title: post_title,
        url: post_url,
        time: post_time,
        source: parsed_url.host_str().unwrap_or(""),
    };
    diesel::insert(&new_link).into(links::table)
        .get_result(conn)
        .expect("Error saving new link")
}

fn crawl_site(connection: &PgConnection, site: &Site) -> i32 {
    let channel = Channel::from_url(&site.url).unwrap();
    let mut latestcheck = site.last_check;
    for item in channel.items() {
        let post_title = item.title().unwrap_or("");
        let post_url = item.link().unwrap_or("");
        let pub_date = item.pub_date().unwrap_or("");
        let post_time = DateTime::parse_from_rfc2822(pub_date).unwrap().timestamp() as i32;
        if post_time > latestcheck {
            latestcheck = post_time;
        }
        if post_time > site.last_check {
            println!("{} : {} : {}", post_title, post_url, post_time);
            insert_link(&connection, post_url, post_title, post_time);
        }
    }
    latestcheck
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
    println!("Scanning {} sites", results.len());
    for site in results {
        let last_check_time = crawl_site(&connection, &site);
        save_last_check(&connection, &site, last_check_time);
    }
}
