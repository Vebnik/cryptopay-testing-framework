

pub mod password {
    use std::error::Error;
    use std::io;

    use argon2::password_hash::SaltString;
    use argon2::{Argon2, PasswordHasher};

    pub async fn hash(password: &str) -> Result<String, Box<dyn Error>> {
        let salt = SaltString::generate(rand::thread_rng());

        let pass = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?
            .to_string();

        Ok(pass)
    }
}

pub mod user {
    use std::{error::Error, sync::Arc};

    use crate::config::State;

    pub async fn check_exist_system_user(state: Arc<State>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}