extern crate serde_json;
extern crate diesel;

use super::schema::links;
use super::schema::users;
use super::schema::auth_tokens;
use diesel::pg::PgConnection;
use self::diesel::prelude::*;

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

#[table_name="users"]
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone)]
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

pub const AUTH_TOKEN_TTL: i64 = 30 * 24 * 60 * 60; // 30 days

#[cfg(test)]
mod tests {
    use super::*;
    use establish_connection;
    use schema::users;

    fn conn_with_a_user() -> PgConnection {
        let connection = establish_connection();
        connection.begin_test_transaction().unwrap();

        let user = User {
            id: 42,
            username: "username".to_string(),
            email: "email@test.com".to_string(),
            password: "passwd".to_string(),
            enable: 1,
        };

        let _ : User = diesel::insert(&user)
                              .into(users::table)
                              .get_result(&connection)
                              .unwrap();

        connection
    }

    #[test]
    fn find_ok() {
        let conn = conn_with_a_user();

        let result = User::find(&conn, 42);
        assert!(result.is_ok());

        let user = result.unwrap();
        assert!(42 == user.id);
    }

    #[test]
    fn find_err() {
        let conn = conn_with_a_user();

        let result = User::find(&conn, 43);
        assert!(result.is_err());
    }

    #[test]
    fn find_by_login_ok() {
        let conn = conn_with_a_user();

        let result = User::find_by_login(&conn, "username", "passwd");
        assert!(result.is_ok());

        let user = result.unwrap();
        assert!("username" == user.username);
    }

    #[test]
    fn find_by_login_err() {
        let conn = conn_with_a_user();

        let result = User::find_by_login(&conn, "non_existing", "passwd");
        assert!(result.is_err());
    }
}
