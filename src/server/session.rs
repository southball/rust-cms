use crate::database::models::User;
use crate::error::SendError;
use crate::server::State;
use diesel::pg::PgConnection;
use tide::{http::Cookie, Response};

const SESSION_ID_KEY: &'static str = "SESSION_ID";

pub fn create_session(
    req: &tide::Request<State>,
    mut res: tide::Response,
    username: &str,
) -> Result<tide::Response, SendError> {
    use crate::database::models::*;
    use crate::database::sessions::create_session as db_create_session;

    let conn = req
        .state()
        .pool
        .get()
        .map_err::<SendError, _>(|err| err.to_string().into())
        .unwrap();
    let session = db_create_session(
        &conn,
        &NewSession {
            session_id: uuid::Uuid::new_v4().to_string(),
            username: username.to_string(),
            expiry: chrono::Utc::now().naive_utc() + chrono::Duration::days(7),
        },
    )?;

    res.set_cookie(
        Cookie::build(SESSION_ID_KEY, session.session_id)
            .path("/")
            .finish(),
    );

    Ok(res)
}

pub fn get_session(
    req: &tide::Request<State>,
) -> Result<Option<crate::database::models::Session>, SendError> {
    let conn = req
        .state()
        .pool
        .get()
        .map_err::<SendError, _>(|err| err.to_string().into())?;

    let session_id = req
        .cookie(SESSION_ID_KEY)
        .map(|cookie| cookie.value().to_string())
        .unwrap_or("".to_string());
    let session =
        crate::database::sessions::get_session_by_id(&conn, &session_id)?
            .and_then(|session| {
                if session.expiry >= chrono::Utc::now().naive_utc() {
                    Some(session)
                } else {
                    None
                }
            });

    Ok(session)
}

pub fn get_session_user(
    req: &tide::Request<State>,
) -> Result<Option<crate::database::models::User>, SendError> {
    let session = get_session(req)?;
    Ok(session.map(|session| {
        let conn = req.state().pool.get().unwrap();
        crate::database::authentication::get_user(&conn, &session.username)
            .unwrap()
    }))
}

pub fn destroy_session(mut res: tide::Response) -> tide::Response {
    res.remove_cookie(Cookie::build(SESSION_ID_KEY, "").path("/").finish());
    res
}

pub fn check_request_admin(
    req: &tide::Request<crate::server::State>,
) -> Result<bool, SendError> {
    let user = get_session_user(req)?;
    Ok(user.map(|user| user.is_admin).unwrap_or(false))
}

#[macro_export]
macro_rules! require_admin {
    ($req:expr) => {{
        {
            use crate::server::templates::render_template;
            let is_admin = crate::server::session::check_request_admin($req)?;

            if !is_admin {
                return render_template(
                    $req,
                    "error.liquid",
                    &liquid::object!({
                        "title": "Forbidden",
                        "body": "You are forbidden from accessing this page."
                    }),
                    tide::StatusCode::Forbidden
                );
            }
        }
    }};
}
