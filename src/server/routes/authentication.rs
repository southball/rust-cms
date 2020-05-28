use crate::database::config;
use crate::handle_body;
use crate::server::templates::render_template;
use crate::server::{ErrorResponse, State};
use serde::{Deserialize, Serialize};
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

pub async fn register_form(mut req: Request<State>) -> Result {
    let conn = req.state().pool.get()?;

    if config::config_open_registration().get(&conn)? {
        render_template(
            &req,
            "register.liquid",
            &liquid::object!({}),
            tide::StatusCode::Ok,
        )
    } else {
        render_template(
            &req,
            "error.liquid",
            &liquid::object!({
                "title": "Error",
                "body": "Register is not open."
            }),
            tide::StatusCode::Forbidden,
        )
    }
}

pub async fn login_form(mut req: Request<State>) -> Result {
    render_template(
        &req,
        "login.liquid",
        &liquid::object!({}),
        tide::StatusCode::Ok,
    )
}

pub async fn register(mut req: Request<State>) -> Result {
    let conn = req.state().pool.get()?;
    let body: RegisterBody = handle_body!(req.body_form().await);

    // Registration is allowed if the 'open registration' setting is on.
    let is_registration_open = config::config_open_registration().get(&conn)?;

    // Registration is allowed if the database contains no users.
    let no_users =
        crate::database::authentication::get_all_users(&conn)?.len() == 0;

    let can_register = is_registration_open || no_users;
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
    let is_user_admin = no_users;

    let _user = crate::database::authentication::create_user(
        &conn,
        &body.username,
        &body.display_name,
        &body.password,
        is_user_admin,
    );

    render_template(
        &req,
        "message.liquid",
        &liquid::object!({
            "title": "Registration Success",
            "body": "You have successfully registered.",
        }),
        tide::StatusCode::Ok,
    )
}

pub async fn login(mut req: Request<State>) -> Result {
    let conn = req.state().pool.get()?;
    let body: LoginBody = handle_body!(req.body_form().await);

    let result = crate::database::authentication::verify_user(
        &conn,
        &body.username,
        &body.password,
    );

    if result {
        let user =
            crate::database::authentication::get_user(&conn, &body.username)
                .unwrap();

        Ok(Response::redirect("/")).map(|res| {
            crate::server::session::create_session(&req, res, &body.username)
                .unwrap()
        })
    } else {
        render_template(
            &req,
            "login.liquid",
            &liquid::object!({
                "error": "Wrong username or password."
            }),
            tide::StatusCode::Ok,
        )
    }
}

pub async fn logout(mut req: Request<State>) -> Result {
    let mut res = Response::redirect("/");
    Ok(crate::server::session::destroy_session(res))
}
