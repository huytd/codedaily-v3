use super::super::schema::users;
use diesel::pg::PgConnection;
use diesel::prelude::*;

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
        use super::super::schema::users::dsl;

        let found_user = dsl::users.filter(dsl::id.eq(id))
            .load::<User>(conn).ok().unwrap().pop();

        if found_user.is_some() {
            Ok(found_user.unwrap())
        } else {
            Err(())
        }
    }

    pub fn find_by_login(conn: &PgConnection, t_username: &str, t_password: &str) -> Result<User, ()> {
        use super::super::schema::users::dsl::*;

        let found_user = users.filter(username.eq(t_username).and(password.eq(t_password)))
            .load::<User>(conn).ok().unwrap().pop();

        if found_user.is_some() {
            Ok(found_user.unwrap())
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_helpers::*;

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
