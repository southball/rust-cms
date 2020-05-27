use super::models::*;
use super::schema::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::query_dsl::RunQueryDsl;

mod hash;

/// Create a new user in the database with the given values.
pub fn create_user(
    conn: &PgConnection,
    username: &str,
    display_name: &str,
    password: &str,
) -> Result<User, Box<dyn std::error::Error>> {
    let salted_hash = hash::generate(&password).unwrap();

    let user = diesel::insert_into(users::table)
        .values(&NewUser {
            display_name: display_name,
            username: username,
            password_hash: &salted_hash.hash,
            password_salt: &salted_hash.salt,
        })
        .get_result::<User>(conn)?;

    Ok(user)
}

/// Finds a user with the given username. If the user is not found, none is
/// returned.
pub fn get_user(conn: &PgConnection, username: &str) -> Option<User> {
    match users::dsl::users
        .filter(users::username.eq(username))
        .first::<User>(conn)
    {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}

/// Verify the password of a user.
pub fn verify_user(
    conn: &PgConnection,
    username: &str,
    password: &str,
) -> bool {
    let user = match get_user(conn, username) {
        Some(user) => user,
        None => {
            return false;
        }
    };

    let salted_hash = hash::SaltedHash {
        hash: user.password_hash,
        salt: user.password_salt,
    };

    let result = hash::verify(&salted_hash, password)
        .or::<Result<bool, ()>>(Ok(false))
        .unwrap();

    result
}