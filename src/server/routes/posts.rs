use crate::database::models::Post;
use crate::server::templates::render_template;
use crate::server::State;
use tide::{Request, Response, Result, StatusCode};

pub async fn get_post(mut req: Request<State>) -> Result {
    let conn = req.state().pool.get()?;
    let slug: String = req.param("slug").unwrap_or("".to_string());
    let post = crate::database::posts::get_post_by_slug(&conn, &slug)?;

    match post {
        Some(post) => render_template(
            &req.state().templates_path.join("post.liquid"),
            &req.state().templates_path.join("partials"),
            &liquid::object!({ "post": post }),
        ),
        None => render_template(
            &req.state().templates_path.join("404.liquid"),
            &req.state().templates_path.join("partials"),
            &liquid::object!({}),
        ),
    }
}

pub async fn get_posts(mut req: Request<State>) -> Result {
    let conn = req.state().pool.get()?;
    let path = &req.state().templates_path;

    let posts: Vec<Post> = crate::database::posts::get_all_posts(&conn)?
        .into_iter()
        .filter(|post| {
            !post.draft && post.publish_time <= chrono::Utc::now().naive_utc()
        })
        .collect();

    let globals = liquid::object!({
        "posts": posts,
    });

    render_template(
        &path.join("posts.liquid"),
        &path.join("partials"),
        &globals,
    )
}
