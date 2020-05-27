use crate::handle_body;
use crate::server::{ErrorResponse, State};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tide::{Request, Response, Result, StatusCode};

#[derive(Serialize, Deserialize)]
struct RegisterBody {
    username: String,
    display_name: String,
    password: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    success: bool,
}

#[derive(Serialize, Deserialize)]
struct LoginBody {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    success: bool,
}

pub async fn register(mut req: Request<State>) -> Result {
    let conn = req.state().pool.get()?;
    let body: RegisterBody = handle_body!(req.body_json().await);

    if let Some(_user) =
        crate::database::authentication::get_user(&conn, &body.username)
    {
        return Ok(Response::new(StatusCode::BadRequest).body_json(
            &ErrorResponse {
                error: "The username is already used.",
            },
        )?);
    }

    let _user = crate::database::authentication::create_user(
        &conn,
        &body.username,
        &body.display_name,
        &body.password,
    );

    Ok(Response::new(StatusCode::Ok)
        .body_json(&RegisterResponse { success: true })?)
}

pub async fn login(mut req: Request<State>) -> Result {
    let conn = req.state().pool.get()?;
    let body: LoginBody = handle_body!(req.body_json().await);

    let result = crate::database::authentication::verify_user(
        &conn,
        &body.username,
        &body.password,
    );

    if result {
        Ok(Response::new(StatusCode::Ok)
            .body_json(&LoginResponse { success: true })?)
    } else {
        Ok(Response::new(StatusCode::Unauthorized).body_json(
            &ErrorResponse {
                error: "Wrong username or password.",
            },
        )?)
    }
}
