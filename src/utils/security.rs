use rand_core::OsRng;
use argon2::password_hash::{self, SaltString};
use argon2::{
    Argon2, 
    PasswordHasher, 
    Algorithm, 
    Version, 
    Params,
    PasswordHash, 
    PasswordVerifier
};

// return hashed password
pub fn compute_password_hash(password: &String) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(OsRng);
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.as_bytes(), &salt)?
    .to_string();
    Ok(password_hash)
}

// verify hashed password 
pub  fn verify_password_hash(
    password: String,
    expected_password_hash: String,
) -> Result<(), password_hash::Error> {

    let expected_password_hash = PasswordHash::new(expected_password_hash.as_str())?;
    Argon2::default().verify_password(password.as_bytes(), &expected_password_hash)

}