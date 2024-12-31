use std::{fmt::Display, sync::LazyLock};

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Password {
    hashed_password: String,
    salt: String,
}

static ARGON2: LazyLock<Argon2<'static>> = LazyLock::new(|| {
    Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        Params::default(),
    )
});

impl Password {
    pub fn new(raw_password: &String) -> Self {
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

    pub fn is_same(&self, raw_password: &String) -> bool {
        let hashed_password = PasswordHash::new(&self.hashed_password).unwrap();

        ARGON2
            .verify_password(raw_password.as_bytes(), &hashed_password)
            .is_ok()
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hashed_password)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_password() {
        let raw_password = "password".to_string();
        let password = Password::new(&raw_password);

        let result = password.is_same(&raw_password);

        assert!(result);
    }

    #[test]
    fn verify_invalid_password() {
        let raw_password = "password".to_string();
        let invalid_password = "invalid password".to_string();
        let password = Password::new(&raw_password);

        let result = password.is_same(&invalid_password);

        assert_eq!(result, false)
    }
}
