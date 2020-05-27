use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::clone::Clone;
use std::path::PathBuf;

#[derive(Clone)]
pub struct State {
    pub pool: Pool<ConnectionManager<PgConnection>>,
    pub templates_path: PathBuf,
    pub jwt_secret: String,
}
