use super::models::*;
use super::schema::*;
use crate::error::SendError;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn create_post(
    conn: &PgConnection,
    new_post: &NewPost,
) -> Result<Post, SendError> {
    diesel::insert_into(posts::table)
        .values(new_post)
        .get_result::<Post>(conn)
        .map_err(|err| err.to_string().into())
}

pub fn get_post_by_slug(conn: &PgConnection, slug: &str) -> Result<Option<Post>, SendError> {
    posts::dsl::posts
        .filter(posts::slug.eq(slug))
        .first::<Post>(conn)
        .optional()
        .map_err(|err| err.to_string().into())
}

pub fn get_all_posts(conn: &PgConnection) -> Result<Vec<Post>, SendError> {
    posts::dsl::posts
        .load::<Post>(conn)
        .map_err(|err| err.to_string().into())
}

pub fn get_posts_by_tag(conn: &PgConnection, tag: &str) -> Result<Vec<Post>, SendError> {
    posts::table
        .left_join(tags::table.on(posts::id.eq(tags::post_id)))
        .filter(tags::tag_name.eq(tag))
        .select((
            posts::id,
            posts::draft,
            posts::publish_time,
            posts::slug,
            posts::title,
            posts::content,
            posts::author
        ))
        .load::<Post>(conn)
        .map_err(|err| err.to_string().into())
}
