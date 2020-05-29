use crate::database::models::Post;
use crate::server::templates::render_template;
use crate::server::State;
use tide::{Request, Response, Result, StatusCode};

pub async fn get_post(mut req: Request<State>) -> Result {
    let conn = req.state().pool.get()?;
    let slug: String = req.param("slug").unwrap_or("".to_string());
    let post = crate::database::posts::get_post_by_slug(&conn, &slug)?
        .map(|post| (post.clone(), crate::database::tags::get_tags_from_post(&conn, &post).unwrap_or(vec![])));

    match post {
        Some((post, post_tags)) => render_template(
            &req,
            "post.liquid",
            &liquid::object!({ "post": post, "post_tags": post_tags }),
            tide::StatusCode::Ok,
        ),
        None => render_template(
            &req,
            "error.liquid",
            &liquid::object!({
                "title": "404 Not Found",
                "body": "Post not found."
            }),
            tide::StatusCode::NotFound,
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

    render_template(
        &req,
        "posts.liquid",
        &liquid::object!({
            "posts": posts,
        }),
        tide::StatusCode::Ok,
    )
}