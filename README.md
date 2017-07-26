# Codedaily 

Live demo: https://news.kipalog.com/

## Setup

### Install Rust

Follow instructions [install rust](https://www.rust-lang.org/en-US/install.html)

### DB

Use Postgres
 - Create your db name and your credentials
 - Put theme into url like: `postgres://username:password@localhost/database_name`
 - Add url into `.env` file

### Config DB

Copy file `.env.example` to `.env` and add your config db url.

### Install deps

Run command: `cargo install`

### Install diesel-cli

Run command: `cargo install diesel_cli --no-default-features --features=postgresql`

### Migrations DB

Run command: `diesel migration run`

### Run local development server

`cargo run --bin server`

### Build release

`cargo build --release`

## Setup frontend

Use yarn

Run command: `yarn install && yarn watch`

## Deploy

Config cronjob

In command line run: `crontab -e` and add job `0 */1 * * * /path/to/bin/crawler`.

This job will run pass every hour at minute 0.


# LICENSE

MIT & APACHE-2


