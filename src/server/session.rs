use crate::database::models::User;
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
        }
        Err(err) => Err(err.into()),
    }
}
