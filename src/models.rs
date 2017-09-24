extern crate serde_json;
extern crate diesel;

use super::schema::links;
use super::schema::users;
use super::schema::auth_tokens;
use diesel::pg::PgConnection;
use self::diesel::prelude::*;
use self::diesel::associations::HasTable;

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

impl User {
    pub fn find(conn: &PgConnection, id: i32) -> Result<User, ()> {
        use schema::users;
        use super::schema::users::dsl;

        let found_users = dsl::users.filter(dsl::id.eq(id))
            .load::<User>(conn).ok().unwrap();

        if found_users.len() > 0 {
            return Ok(found_users.first().unwrap().clone());
        } else {
            return Err(());
        }
    }

    pub fn find_by_login(conn: &PgConnection, t_username: &str, t_password: &str) -> Result<User, ()> {
        use schema::users;
        use super::schema::users::dsl::*;

        let found_users = users.filter(username.eq(t_username).and(password.eq(t_password)))
            .load::<User>(conn).ok().unwrap();

        if found_users.len() > 0 {
            return Ok(found_users.first().unwrap().clone());
        } else {
            return Err(());
        }
    }
}

#[table_name="auth_tokens"]
#[derive(Queryable, Serialize, Insertable, Deserialize, Debug, Clone)]
pub struct AuthToken {
    pub token: String,
    pub user_id: i32,
    pub expired_at: i64,
}
