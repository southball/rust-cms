use super::schema::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub draft: bool,
    pub publish_time: chrono::NaiveDateTime,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub author: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct NewPost {
    pub draft: bool,
    pub publish_time: chrono::NaiveDateTime,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub author: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub display_name: String,
    pub password_salt: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub last_update: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub display_name: String,
    pub password_salt: String,
    pub password_hash: String,
    pub is_admin: bool,
}

#[derive(Queryable)]
pub struct Config {
    pub config_name: String,
    pub config_value: String,
}

#[derive(Insertable)]
#[table_name = "config"]
pub struct NewConfig {
    pub config_name: String,
    pub config_value: String,
}
