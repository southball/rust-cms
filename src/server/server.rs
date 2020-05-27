use super::pages;
use super::state::State;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::path::PathBuf;
use tide::Server;

pub fn create_server(
    pool: Pool<ConnectionManager<PgConnection>>,
    templates_path: PathBuf,
) -> Server<State> {
    let state = State {
        pool,
        templates_path,
    };

    let mut server = tide::with_state(state.clone());

    server.at("/").get(pages::index::index);

    server.at("/auth").nest({
        let mut api = tide::with_state(state.clone());
        api.at("/login").post(pages::authentication::login);
        api.at("/register").post(pages::authentication::register);
        api
    });

    server
}
