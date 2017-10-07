use super::super::schema::comments;

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Comment {
    pub id: i32,
    pub message: String,
    pub link_id: i32,
    pub author_id: i32,
}

#[table_name="comments"]
#[derive(Serialize, Insertable, Debug, Clone)]
pub struct NewComment {
    pub message: String,
    pub link_id: i32,
    pub author_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PostComment {
    pub message: String,
}
