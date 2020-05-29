use crate::server::templates::render_template;
use crate::server::State;
use crate::unwrap_or_error;
use tide::{Request, Result};

pub async fn all_tags_page(mut req: Request<State>) -> Result {
    let conn = req.state().pool.get()?;
    let tags_with_count =
        crate::database::tags::get_public_tags_with_count(&conn)?;

    render_template(
        &req,
        "tags.liquid",
        &liquid::object!({ "tags": tags_with_count }),
        tide::StatusCode::Ok,
    )
}

pub async fn single_tag_page(mut req: Request<State>) -> Result {
    let conn = req.state().pool.get()?;
    let tag = req.param("tag").unwrap_or("".to_string());
    let posts = crate::database::posts::get_posts_by_tag(&conn, &tag)?;

    render_template(
        &req,
        "tag.liquid",
        &liquid::object!({
            "tag": &tag,
            "posts": &posts,
        }),
        tide::StatusCode::Ok,
    )
}
