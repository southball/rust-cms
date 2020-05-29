use super::schema::*;
use std::clone::Clone;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub draft: bool,
    pub publish_time: chrono::NaiveDateTime,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub author: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct NewPost {
    pub draft: bool,
    pub publish_time: chrono::NaiveDateTime,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub author: String,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub display_name: String,
    pub password_salt: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub last_update: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub display_name: String,
    pub password_salt: String,
    pub password_hash: String,
    pub is_admin: bool,
}

#[derive(Debug, Queryable)]
pub struct Config {
    pub config_name: String,
    pub config_value: String,
}

#[derive(Debug, Insertable)]
#[table_name = "config"]
pub struct NewConfig {
    pub config_name: String,
    pub config_value: String,
}

#[derive(Debug, Queryable)]
pub struct Session {
    pub session_id: String,
    pub username: String,
    pub expiry: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "sessions"]
pub struct NewSession {
    pub session_id: String,
    pub username: String,
    pub expiry: chrono::NaiveDateTime,
}

#[derive(Debug, Queryable)]
pub struct Tag {
    pub tag_name: String,
    pub post_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "tags"]
pub struct NewTag {
    pub tag_name: String,
    pub post_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TagWithCount {
    pub tag_name: String,
    pub count: i64,
}
