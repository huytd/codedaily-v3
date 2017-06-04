use super::schema::links;

#[derive(Queryable)]
pub struct Site {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub last_check: i32,
}

#[derive(Queryable)]
#[derive(Serialize)]
pub struct Link {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub body: Option<String>,
    pub time: i32,
    pub source: Option<String>,
}

#[derive(Insertable)]
#[table_name="links"]
pub struct NewLink<'a> {
    pub title: &'a str,
    pub url: &'a str,
    pub time: i32,
    pub source: &'a str,
}
