use crate::server::templates::render_template;
use crate::server::State;
use tide::{Request, Response, Result, StatusCode};

pub async fn index(mut req: Request<State>) -> Result {
    render_template(
        &req,
        "admin/index.liquid",
        &liquid::object!({}),
        tide::StatusCode::Ok,
    )
}
