use std::{fmt::Display, sync::LazyLock};

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, Params, PasswordHasher,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Password {
    hashed_password: String,
    salt: String,
}

impl Password {
    pub fn new(raw_password: String) -> Self {
        static ARGON2: LazyLock<Argon2> = LazyLock::new(|| {
            Argon2::new(
                argon2::Algorithm::Argon2id,
                argon2::Version::V0x13,
                Params::default(),
            )
        });

        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = ARGON2
            .hash_password(raw_password.as_bytes(), &salt)
            .unwrap();

        Self {
            hashed_password: hashed_password.to_string(),
            salt: salt.to_string(),
        }
    }

    pub fn new_with(hashed_password: String, salt: String) -> Self {
        Self {
            hashed_password,
            salt,
        }
    }

    pub fn salt(&self) -> String {
        self.salt.to_string()
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hashed_password)?;
        Ok(())
    }
}
