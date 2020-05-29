use super::routes;
use super::state::State;
use tide::Server;

pub fn create_server(state: State) -> Server<State> {
    let mut server = tide::with_state(state.clone());

    server.at("/").get(routes::index::index);

    server.at("/admin").nest({
        let mut api = tide::with_state(state.clone());
        api.at("/").get(routes::admin::index::index);
        api.at("/post/:slug")
            .get(routes::admin::posts::edit_post_get);
        api.at("/posts")
            .get(routes::admin::posts::get_all_posts)
            .post(routes::admin::posts::create_post);
        api.at("/config").get(routes::admin::config::config_page);
        api
    });

    server.at("/auth").nest({
        let mut api = tide::with_state(state.clone());
        api.at("/login")
            .get(routes::authentication::login_form)
            .post(routes::authentication::login);
        api.at("/register")
            .get(routes::authentication::register_form)
            .post(routes::authentication::register);
        api.at("/logout").get(routes::authentication::logout);
        api
    });

    server.at("/posts").get(routes::posts::get_posts);
    server.at("/post/:slug").get(routes::posts::get_post);
    server.at("/tags").get(routes::tags::all_tags_page);
    server.at("/tag/:tag").get(routes::tags::single_tag_page);

    // The custom 404 handler is disabled due to Tide not supporting custom 404
    // handler at the moment.
    //
    // server.at("*").get(routes::not_found::not_found);

    server
}
