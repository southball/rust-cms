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
) -> tide::Result {
    use tide::{Response, StatusCode};

    let template = liquid::ParserBuilder::with_stdlib()
        .partials(liquid::partials::OnDemandCompiler::<PartialSource>::new(
            PartialSource::new(partials_path).unwrap(),
        ))
        .build()?
        .parse_file(&template_path)?;
    let response = template.render(globals)?;

    Ok(Response::new(StatusCode::Ok)
        .body_string(response)
        .set_mime(mime::TEXT_HTML_UTF_8))
}

/// A function that renders the template as response. This should be returned
/// from a route.
pub fn render_template(
    template_path: &std::path::Path,
    partials_path: &std::path::Path,
    globals: &liquid::Object,
) -> tide::Result {
    match inner_render_template(template_path, partials_path, globals) {
        Ok(response) => {
            println!("Successfully rendered template at {:?}", template_path);   
            Ok(response)
        },
        Err(err) => {
            println!("Error when rendering template: {:?}", err);
            Err(err.into())
        }
    }
}
