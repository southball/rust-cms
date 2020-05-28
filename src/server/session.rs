use crate::database::models::User;
use crate::error::SendError;
use diesel::pg::PgConnection;

mod jwt;

pub use jwt::JWT;

pub fn generate_token_for_user(secret: &str, user: &User) -> JWT {
    jwt::generate_token(secret, &user.username, user.is_admin)
}

pub fn get_user_from_token(
    conn: &PgConnection,
    secret: &str,
    token: &str,
) -> Result<Option<User>, Box<dyn std::error::Error>> {
    match jwt::decode_token(secret, token) {
        Ok(token_data) => {
            let claims: jwt::JWTClaims = token_data.claims;
            Ok(crate::database::authentication::get_user(conn, &claims.sub))
        },
        Err(err) => Err(err.into()),
    }
}

pub fn check_request_admin(
    req: &tide::Request<crate::server::State>,
) -> Result<bool, SendError> {
    let conn = req
        .state()
        .pool
        .get()
        .map_err(|err| SendError::from(err.to_string()))?;
    let secret = &req.state().jwt_secret;

    match req.header("Authorization") {
        Some(authentication) => {
            let key = authentication.get(0)
                .map(|header| header.to_string())
                .unwrap_or("".to_string());
            if key.starts_with("Bearer ") {
                let token = key[7..].to_string();
                let user = get_user_from_token(&conn, secret, &token)
                    .map_err(|err| SendError::from(err.to_string()))?;
                Ok(user.map(|user| user.is_admin).unwrap_or(false))
            } else {
                Ok(false)
            }
        }
        None => Ok(false),
    }
}

#[macro_export]
macro_rules! require_admin {
    ($req:expr) => {{
        {
            use crate::server::error::ErrorResponse;
            use tide::{Response, StatusCode};
            let is_admin = crate::server::session::check_request_admin($req)?;

            if !is_admin {
                return Ok(Response::new(StatusCode::Forbidden).body_json(
                    &ErrorResponse {
                        error:
                            "You cannot access this page as you are not admin.",
                    },
                )?);
            }
        }
    }};
}
