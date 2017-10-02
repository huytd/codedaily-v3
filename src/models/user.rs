use super::super::schema::users;
use diesel::pg::PgConnection;
use diesel::prelude::*;

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
        use super::super::schema::users::dsl;

        let found_users = dsl::users.filter(dsl::id.eq(id))
            .load::<User>(conn).ok().unwrap();

        if found_users.len() > 0 {
            return Ok(found_users.first().unwrap().clone());
        } else {
            return Err(());
        }
    }

    pub fn find_by_login(conn: &PgConnection, t_username: &str, t_password: &str) -> Result<User, ()> {
        use super::super::schema::users::dsl::*;

        let found_users = users.filter(username.eq(t_username).and(password.eq(t_password)))
            .load::<User>(conn).ok().unwrap();

        if found_users.len() > 0 {
            return Ok(found_users.first().unwrap().clone());
        } else {
            return Err(());
        }
    }
}