#![feature(const_nonzero_int_methods)]
use server::State;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod database;
mod server;
pub mod error;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();

    // Establish connection to database.
    let pool = {
        let database_url = std::env::var("DATABASE_URL")
            .expect("The environment variable DATABASE_URL must be set");
        database::connection::establish_connection(&database_url)
    };

    // Read other environment variables
    let listen_interface = std::env::var("LISTEN")
        .expect("The environment variable LISTEN must be set");
    let templates_path: std::path::PathBuf = std::env::var("TEMPLATES")
        .expect("The environment variable TEMPLATES_PATH must be set.")
        .into();
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("The environmental variable JWT_SECRET must be set.");

    // Construct server with connection pool.
    let server = crate::server::create_server(State {
        pool,
        templates_path,
        jwt_secret,
    });
    server.listen(&listen_interface).await?;

    Ok(())
}
