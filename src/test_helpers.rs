
use super::*;
use establish_connection;
use schema::users;
use models::*;

pub fn conn_with_a_user() -> PgConnection {
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
