use std::borrow::Cow;

/// This is a PartialSource for liquid partials.
#[derive(Debug)]
pub struct PartialSource {
    pub path: std::path::PathBuf,
    pub names: Vec<String>,
}

impl PartialSource {
    fn names(
        path: &std::path::Path,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        Ok(path
            .read_dir()?
            .map(|entry| {
                entry.map(|file| {
                    file.path().strip_prefix(path).unwrap().to_path_buf()
                })
            })
            .filter_map(|result| result.map_or(None, |p| Some(p)))
            .map(|p| p.to_str().unwrap().to_string())
            .collect())
    }

    pub fn new(
        path: &std::path::Path,
    ) -> Result<PartialSource, Box<dyn std::error::Error>> {
        Ok(PartialSource {
            path: path.to_owned(),
            names: PartialSource::names(path)?,
        })
    }
}

impl liquid::partials::PartialSource for PartialSource {
    fn contains(&self, name: &str) -> bool {
        self.names().iter().any(|&s| s == name)
    }

    fn names(&self) -> Vec<&str> {
        self.names.iter().map(|s| &**s).collect()
    }

    fn try_get<'a>(&'a self, name: &str) -> Option<Cow<'a, str>> {
        let path = self.path.join(name);

        if !path.is_file() {
            None
        } else {
            Some(std::fs::read_to_string(&path).unwrap().into())
        }
    }
}

pub fn inner_render_template(
    template_path: &std::path::Path,
    partials_path: &std::path::Path,
    globals: &liquid::Object,
    status_code: tide::StatusCode,
) -> tide::Result {
    use tide::{Response, StatusCode};

    let template = liquid::ParserBuilder::with_stdlib()
        .partials(liquid::partials::OnDemandCompiler::<PartialSource>::new(
            PartialSource::new(partials_path).unwrap(),
        ))
        .build()?
        .parse_file(&template_path)?;
    let response = template.render(globals)?;

    Ok(Response::new(status_code)
        .body_string(response)
        .set_mime(mime::TEXT_HTML_UTF_8))
}

/// A function that renders the template as response. This should be returned
/// from a route.
pub fn render_template(
    request: &tide::Request<crate::server::State>,
    template_location: &str,
    globals: &liquid::Object,
    status_code: tide::StatusCode,
) -> tide::Result {
    let template_path = request.state().templates_path.join(template_location);
    let partials_path = request.state().templates_path.join("partials");

    let conn = request.state().pool.get()?;
    let session_user = crate::server::session::get_session_user(&request)?;
    let mut variables = liquid::object!({
        "site_name": crate::database::config::config_site_name().get(&conn)?,
        "open_registration": crate::database::config::config_open_registration().get(&conn)?,
        "is_logged_in": session_user.is_some(),
        "is_admin": session_user
                .map(|user| user.is_admin)
                .unwrap_or(false)
    });

    globals.iter().for_each(|(key, value)| {
        variables.insert(key.clone(), value.clone());
    });

    match inner_render_template(
        &template_path,
        &partials_path,
        &variables,
        status_code,
    ) {
        Ok(response) => {
            println!("Successfully rendered template at {:?}", template_path);
            Ok(response)
        }
        Err(err) => {
            println!("Error when rendering template: {:?}", err);
            Err(err.into())
        }
    }
}
