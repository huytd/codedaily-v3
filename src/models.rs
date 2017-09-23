extern crate serde_json;

use super::schema::links;
use super::schema::users;
use super::schema::auth_tokens;

#[derive(Queryable)]
pub struct Site {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub last_check: i32,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Link {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub body: Option<String>,
    pub time: i32,
    pub source: Option<String>,
}

#[table_name="links"]
#[derive(Serialize, Insertable, Debug, Clone)]
pub struct NewLink {
    pub title: String,
    pub url: String,
    pub body: Option<String>,
    pub time: i32,
    pub source: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub enable: i32
}

#[table_name="users"]
#[derive(Serialize, Insertable, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub enable: i32
}

#[table_name="auth_tokens"]
#[derive(Queryable, Serialize, Insertable, Deserialize, Debug, Clone)]
pub struct AuthToken {
    pub token: String,
    pub user_id: i32,
    pub expired_at: i64,
}
