use crate::server::State;
use tide::{Request, Response, Result, StatusCode};

pub async fn not_found(mut req: Request<State>) -> Result {
    crate::server::templates::render_template(
        &req,
        "error.liquid",
        &liquid::object!({
            "title": "404 Not Found",
            "body": "Page not found."
        }),
        tide::StatusCode::NotFound,
    )
}
