use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::error::SendError;
use super::schema::*;
use super::models::*;

pub fn create_session(conn: &PgConnection, session: &NewSession) -> Result<Session, SendError> {
    diesel::insert_into(sessions::table)
        .values(session)
        .get_result::<Session>(conn)
        .map_err(|err| err.to_string().into())
}

pub fn get_session_by_id(conn: &PgConnection, session_id: &str) -> Result<Option<Session>, SendError> {
    sessions::dsl::sessions
        .filter(sessions::session_id.eq(session_id))
        .first::<Session>(conn)
        .optional()
        .map_err(|err| err.to_string().into())
}
