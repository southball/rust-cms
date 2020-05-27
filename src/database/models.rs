use super::schema::*;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub draft: bool,
    pub publish_time: chrono::DateTime<chrono::Utc>,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub author: Option<i32>,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub password_salt: String,
    pub password_hash: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub display_name: &'a str,
    pub password_salt: &'a str,
    pub password_hash: &'a str,
}
