use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ErrorResponse<'a> {
    pub error: &'a str,
}

/// A macro to automatically return 400 bad request when the body parsing fails.
#[macro_export]
macro_rules! handle_body {
    ($body:expr) => {{
        match $body {
            Ok(body) => body,
            Err(_) => {
                return Ok(Response::new(StatusCode::BadRequest).body_json(
                    &ErrorResponse {
                        error: "Failed to parse body.",
                    },
                )?);
            }
        }
    }};
}
