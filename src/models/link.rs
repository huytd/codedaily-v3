use super::super::schema::links;

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