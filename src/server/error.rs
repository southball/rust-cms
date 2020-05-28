use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ErrorResponse<'a> {
    pub error: &'a str,
}

#[macro_export]
macro_rules! unwrap_or_error {
    ($body:expr, $message:expr, $code:expr) => {{
        match $body {
            Ok(body) => body,
            Err(err) => {
                println!("Error when unwrapping variable ({}): {:?}", $message, &err);
                return Ok(tide::Response::new(tide::StatusCode::BadRequest)
                    .body_json(
                    &crate::server::error::ErrorResponse {
                        error: $message,
                    },
                )?);
            }
        }
    }};

    ($body:expr, $message:expr) => {{
        unwrap_or_error!($body, $message, tide::StatusCode::InternalServerError)
    }};

    ($body:expr) => {{
        unwrap_or_error!($body, "Error when unwrapping variable.")
    }};
}

/// A macro to automatically return 400 bad request when the body parsing fails.
#[macro_export]
macro_rules! handle_body {
    ($body:expr) => {{
        use crate::unwrap_or_error;
        unwrap_or_error!($body, "Failed to parse body", tide::StatusCode::BadRequest)
    }};
}