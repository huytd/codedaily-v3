#[derive(Queryable)]
pub struct Site {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub last_check: i32,
}
