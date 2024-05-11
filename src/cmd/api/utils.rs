pub mod password {
    use argon2::password_hash::SaltString;
    use argon2::{Argon2, PasswordHasher};
    use std::io;

    use crate::{Error, Result};

    pub async fn hash(password: &str) -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());

        let pass = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| Error::PasswordHash)?
            .to_string();

        Ok(pass)
    }
}
