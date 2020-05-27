use ring::rand::SecureRandom;
use std::num::NonZeroU32;

pub struct SaltedHash {
    pub salt: String,
    pub hash: String,
}

const CREDENTIAL_LEN: usize = ring::digest::SHA512_OUTPUT_LEN;
const N_ITER: Option<NonZeroU32> = NonZeroU32::new(100_000);

pub fn generate(
    password: &str,
) -> Result<SaltedHash, ring::error::Unspecified> {
    let rng = ring::rand::SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)?;

    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    ring::pbkdf2::derive(
        ring::pbkdf2::PBKDF2_HMAC_SHA512,
        N_ITER.unwrap(),
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );

    Ok(SaltedHash {
        salt: data_encoding::HEXUPPER.encode(&salt),
        hash: data_encoding::HEXUPPER.encode(&pbkdf2_hash),
    })
}

pub fn verify(
    salted_hash: &SaltedHash,
    password: &str,
) -> Result<bool, ring::error::Unspecified> {
    let salt = data_encoding::HEXUPPER
        .decode(salted_hash.salt.as_bytes())
        .unwrap();
    let hash = data_encoding::HEXUPPER
        .decode(salted_hash.hash.as_bytes())
        .unwrap();

    let verification = ring::pbkdf2::verify(
        ring::pbkdf2::PBKDF2_HMAC_SHA512,
        N_ITER.unwrap(),
        &salt,
        password.as_bytes(),
        &hash,
    );

    Ok(verification.is_ok())
}
