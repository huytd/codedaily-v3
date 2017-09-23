extern crate serde_json;

use super::schema::links;
use super::schema::users;
use super::schema::votes;

#[derive(Queryable)]
pub struct Site {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub last_check: i32,
}

#[table_name="links"]
#[derive(Queryable, Serialize, Insertable, Debug, Clone)]
pub struct Link {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub body: Option<String>,
    pub time: i32,
    pub source: Option<String>,
}

#[table_name="users"]
#[derive(Queryable, Serialize, Insertable, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub enable: i32
}

#[table_name="votes"]
#[derive(Queryable, Serialize, Insertable, Deserialize, Debug, Clone)]
pub struct Vote {
    pub id: i32,
    pub link_id: i32,
    pub user_id: i32,
    pub dir: i16,
}

#[table_name="votes"]
#[derive(Insertable)]
pub struct NewVote {
    pub link_id: i32,
    pub dir: i16,
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct VoteParam {
    pub link_id: i32,
    pub dir: i16,
}
