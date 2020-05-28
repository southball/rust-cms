use crate::server::templates::render_template;
use crate::server::State;
use tide::{Request, Result, StatusCode};

pub async fn config_page(mut req: Request<State>) -> Result {
    render_template(
        &req,
        "admin/config.liquid",
        &liquid::object!({}),
        StatusCode::Ok,
    )
}
