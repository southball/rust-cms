use super::routes;
use super::state::State;
use tide::Server;

pub fn create_server(state: State) -> Server<State> {
    let mut server = tide::with_state(state.clone());

    server.at("/").get(routes::index::index);

    server.at("/admin").nest({
        let mut api = tide::with_state(state.clone());
        api.at("/posts")
            .get(routes::admin::posts::get_all_posts)
            .post(routes::admin::posts::create_post);
        api
    });

    server.at("/auth").nest({
        let mut api = tide::with_state(state.clone());
        api.at("/login").post(routes::authentication::login);
        api.at("/register").post(routes::authentication::register);
        api
    });

    server.at("/posts").get(routes::posts::get_posts);
    server.at("/post/:slug").get(routes::posts::get_post);

    server
}
