use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JWT {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    pub sub: String,
    pub exp: u64,
    pub iat: u64,
    pub is_admin: bool,
    pub can_refresh: bool,
}

pub fn generate_token_helper(
    secret: &str,
    username: &str,
    is_admin: bool,
    can_refresh: bool,
) -> String {
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

    let iat: u64 = chrono::Utc::now().timestamp() as u64;
    let exp: u64 = if can_refresh {
        chrono::Utc::now() + chrono::Duration::days(7)
    } else {
        chrono::Utc::now() + chrono::Duration::minutes(20)
    }.timestamp() as u64;

    encode(
        &Header::new(Algorithm::HS256),
        &JWTClaims {
            sub: username.to_string(),
            iat,
            exp,
            is_admin,
            can_refresh,
        },
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

pub fn generate_token(secret: &str, username: &str, is_admin: bool) -> JWT {
    JWT {
        access_token: generate_token_helper(secret, username, is_admin, false),
        refresh_token: generate_token_helper(secret, username, is_admin, true),
    }
}

pub fn decode_token(
    secret: &str,
    token: &str,
) -> Result<jsonwebtoken::TokenData<JWTClaims>, jsonwebtoken::errors::Error> {
    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

    decode::<JWTClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
}
