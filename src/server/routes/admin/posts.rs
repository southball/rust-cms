use crate::handle_body;
use crate::server::templates::render_template;
use crate::server::State;
use crate::{require_admin, unwrap_or_error};
use serde::{Deserialize, Serialize};
use tide::{Request, Response, Result, StatusCode};

#[derive(Serialize, Deserialize)]
pub struct CreatePostNewPost {
    draft: Option<bool>,
    publish_time: Option<i64>,
    slug: String,
    title: String,
    content: String,
    author: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePostBody {
    pub post: CreatePostNewPost,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePostResponse {
    pub post: crate::database::models::Post,
}

pub async fn edit_post_get(mut req: Request<State>) -> Result {
    require_admin!(&req);

    let conn = req.state().pool.get()?;
    let slug: String = req.param("slug").unwrap_or("".to_string());
    let post = crate::database::posts::get_post_by_slug(&conn, &slug)?;

    match post {
        Some(post) => render_template(
            &req,
            "admin/post.liquid",
            &liquid::object!({ "post": post }),
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

pub async fn create_post(mut req: Request<State>) -> Result {
    require_admin!(&req);

    let conn = req.state().pool.get()?;
    let body: CreatePostBody = handle_body!(req.body_json().await);
    let post = crate::database::posts::create_post(
        &conn,
        &crate::database::models::NewPost {
            draft: body.post.draft.unwrap_or(false),
            publish_time: body
                .post
                .publish_time
                .map(|s| chrono::NaiveDateTime::from_timestamp(s, 0))
                .unwrap_or(chrono::Utc::now().naive_utc()),
            slug: body.post.slug,
            title: body.post.title,
            content: body.post.content,
            author: body.post.author,
        },
    )?;

    Ok(
        Response::new(StatusCode::Ok)
            .body_json(&CreatePostResponse { post })?,
    )
}

pub async fn get_all_posts(mut req: Request<State>) -> Result {
    require_admin!(&req);

    let conn = req.state().pool.get()?;
    let posts = crate::database::posts::get_all_posts(&conn)?;
    
    render_template(
        &req,
        "admin/posts.liquid",
        &liquid::object!({ "posts": posts }),
        tide::StatusCode::Ok,
    )
}
