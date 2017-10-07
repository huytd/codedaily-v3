use super::super::schema::auth_tokens;

#[table_name="auth_tokens"]
#[derive(Queryable, Serialize, Insertable, Deserialize, Debug, Clone)]
pub struct AuthToken {
    pub token: String,
    pub user_id: i32,
    pub expired_at: i64,
}

pub const AUTH_TOKEN_TTL: i64 = 30 * 24 * 60 * 60; // 30 days