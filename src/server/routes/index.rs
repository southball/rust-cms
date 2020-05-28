use crate::server::State;
use tide::{Request, Result};

pub async fn index(req: Request<State>) -> Result {
    let conn = req.state().pool.get()?;

    let site_name = crate::database::config::config_site_name().get(&conn)?;
    let globals = liquid::object!({ "site_name": site_name });

    crate::server::templates::render_template(
        &req,
        "index.liquid",
        &globals,
        tide::StatusCode::Ok,
    )
}
