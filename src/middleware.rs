extern crate rocket;
extern crate diesel;

use self::rocket::Outcome;
use self::rocket::http::Status;
use self::rocket::request::{self, Request, FromRequest};
use models::*;
use helpers::*;
use self::diesel::prelude::*;
use establish_connection;
use diesel::pg::PgConnection;
use std::time::{Instant, UNIX_EPOCH};
use std::time::SystemTime;

#[derive(Debug)]
pub struct Auth {
    pub token: String,
    pub user_id: i32,
}

fn authenticate(conn: &PgConnection, t_token: &str) -> Result<Auth, String> {
    use schema::auth_tokens;
    use schema::users;
    use schema::auth_tokens::dsl::*;
    use schema::users::dsl::*;

    let auth_token = auth_tokens.filter(token.eq(t_token))
        .load::<AuthToken>(conn).ok()
        .unwrap();

    if auth_token.len() > 0 {
        let auth_token = auth_token.first().unwrap();
        let now = epoch_now() as i64;

        if now >= auth_token.expired_at {
            return Err(String::from("Token has been expired"));
        } else {
            let _: AuthToken = diesel::update(auth_tokens.find(t_token))
                .set(expired_at.eq(now + 30 * 24 * 60 ^ 60))
                .get_result(conn).expect("Error increasing token expiry");
        }

        let user_existence: i32 = users.filter(users::id.eq(auth_token.user_id))
            .count()
            .get_result(conn)
            .unwrap_or(0) as i32;

        if user_existence == 0 {
            return Err(String::from("User does not exist"));
        }

        let auth = Auth {
            token: auth_token.token.to_string(),
            user_id: auth_token.user_id,
        };
        return Ok(auth);
    } else {
        return Err(String::from("Token is invalid"));
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, ()> {
        let tokens: Vec<_> = request.headers().get("x-authorization").collect();
        if tokens.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let token = tokens.first().unwrap();
        let conn = establish_connection();

        let result = authenticate(&conn, token);

        match result {
            Ok(auth) => return Outcome::Success(auth),
            Err(_msg) => return Outcome::Failure((Status::Unauthorized, ())) // TODO: log the message
        }
    }
}
