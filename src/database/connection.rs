use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn establish_connection(
    database_url: &str,
) -> Pool<ConnectionManager<PgConnection>> {
    let connection_manager =
        ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(connection_manager)
        .expect("Failed to create database connection pool.");

    pool
}
