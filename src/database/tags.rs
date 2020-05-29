use super::models::*;
use super::schema::*;
use crate::error::SendError;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::dsl::*;

/// Get aggregated statistics for tags from posts that are public, i.e.
/// non-draft. only.
pub fn get_public_tags_with_count(
    conn: &PgConnection
) -> Result<Vec<TagWithCount>, SendError> {
    tags::table
        .left_join(posts::table.on(tags::post_id.eq(posts::id)))
        .filter(posts::draft.eq(false))
        .group_by(tags::tag_name)
        .select((tags::tag_name, sql::<diesel::sql_types::BigInt>("COUNT(*)")))
        .load::<(String, i64)>(conn)
        .map(|result| {
            result
                .into_iter()
                .map(|(tag_name, count)| TagWithCount { tag_name, count })
                .collect()
        })
        .map_err(|err| err.to_string().into())
}

/// Get aggregated statistics of all tags, including those from posts that are
/// draft.
pub fn get_all_tags_with_count(
    conn: &PgConnection,
) -> Result<Vec<TagWithCount>, SendError> {
    tags::table
        .left_join(posts::table.on(tags::post_id.eq(posts::id)))
        .group_by(tags::tag_name)
        .select((tags::tag_name, sql::<diesel::sql_types::BigInt>("COUNT(*)")))
        .load::<(String, i64)>(conn)
        .map(|result| {
            result
                .into_iter()
                .map(|(tag_name, count)| TagWithCount { tag_name, count })
                .collect()
        })
        .map_err(|err| err.to_string().into())
}

pub fn get_tags_from_post(conn: &PgConnection, post: &Post) -> Result<Vec<String>, SendError> {
    tags::table
        .filter(tags::post_id.eq(post.id))
        .select(tags::tag_name)
        .load::<String>(conn)
        .map_err(|err| err.to_string().into())
}