use crate::server::State;
use handlebars::Handlebars;
use std::collections::BTreeMap;
use tide::{Request, Response, Result, StatusCode};

pub async fn index(req: Request<State>) -> Result {
    let template_path = req.state().templates_path.join("index.hbs");
    let template = std::fs::read_to_string(template_path)?;

    let mut handlebars = Handlebars::new();
    let response: String =
        handlebars.render_template(&template, &BTreeMap::<(), ()>::new())?;

    Ok(Response::new(StatusCode::Ok).body_string(response))
}
