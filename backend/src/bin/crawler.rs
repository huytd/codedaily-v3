extern crate codedaily_backend;
extern crate diesel;
extern crate rss;

use self::codedaily_backend::*;
use self::codedaily_backend::models::*;
use self::diesel::prelude::*;
use diesel::pg::PgConnection;
use rss::Channel;

fn crawl_site(connection: &PgConnection, url: String) {
    let channel = Channel::from_url(&url).unwrap();
    println!("{:?}", channel);
}

fn main() {
    use codedaily_backend::schema::sites::dsl::*;

    let connection = establish_connection();
    let results = sites.load::<Site>(&connection).expect("Error loading sites");
    println!("Displaying {} sites", results.len());
    for site in results {
        println!("{}", site.name);
        crawl_site(&connection, site.url);
    }
}
