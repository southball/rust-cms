use crate::database::config;
use crate::handle_body;
use crate::server::{ErrorResponse, State};
use serde::{Deserialize, Serialize};
use tide::{Request, Response, Result, StatusCode};

#[derive(Serialize, Deserialize)]
struct RegisterBody {
    username: String,
    display_name: String,
    password: String,
    token: Option<String>,
    is_admin: Option<bool>,
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

    // The admin can always register additional users.
    let is_registrant_admin =
        match body.token {
            Some(token) => crate::server::session::get_user_from_token(
                &conn, &req.state().jwt_secret, &token,
            )
            .map(|user_opt| user_opt.map(|user| user.is_admin).unwrap_or(false))
            .unwrap_or(false),
            None => false,
        };

    // Registration is allowed if the 'open registration' setting is on.
    let is_registration_open = config::config_open_registration().get(&conn)?;

    // Registration is allowed if the database contains no users.
    let no_users =
        crate::database::authentication::get_all_users(&conn)?.len() == 0;

    let can_register = is_registrant_admin || is_registration_open || no_users;
    if !can_register {
        return Ok(Response::new(StatusCode::BadRequest).body_json(
            &ErrorResponse {
                error: "You do not have permission to register.",
            },
        )?);
    }

    // Check whether the username is already used.
    if let Some(_user) =
        crate::database::authentication::get_user(&conn, &body.username)
    {
        return Ok(Response::new(StatusCode::BadRequest).body_json(
            &ErrorResponse {
                error: "The username is already used.",
            },
        )?);
    }

    // Determine whether the new registered user is admin.
    let is_user_admin =
        no_users || (is_registrant_admin && body.is_admin == Some(true));

    let _user = crate::database::authentication::create_user(
        &conn,
        &body.username,
        &body.display_name,
        &body.password,
        is_user_admin,
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
        let user =
            crate::database::authentication::get_user(&conn, &body.username).unwrap();

        Ok(Response::new(StatusCode::Unauthorized).body_json(
            &crate::server::session::generate_token_for_user(&req.state().jwt_secret, &user),
        )?)
    } else {
        Ok(Response::new(StatusCode::Unauthorized).body_json(
            &ErrorResponse {
                error: "Wrong username or password.",
            },
        )?)
    }
}
