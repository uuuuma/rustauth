use std::{fmt::Display, ops::Not, sync::LazyLock};

use anyhow::anyhow;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
};
use regex::Regex;

use crate::error::DomainError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Password {
    hashed_password: String,
}

static ARGON2: LazyLock<Argon2<'static>> = LazyLock::new(|| {
    Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        Params::default(),
    )
});

impl Password {
    pub fn new(raw_password: &String) -> Result<Self, Vec<DomainError>> {
        Self::validate(raw_password)?;

        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = match ARGON2.hash_password(raw_password.as_bytes(), &salt) {
            Ok(password) => password.to_string(),
            Err(err) => return Err(vec![DomainError::UnexpectedError(anyhow!(err))]),
        };

        Ok(Self { hashed_password })
    }

    fn validate(raw_password: &str) -> Result<(), Vec<DomainError>> {
        let mut errors = vec![];

        if raw_password.is_ascii().not() {
            errors.push(DomainError::ValidationError {
                description: "use only ascii characters for password",
            });
        }

        let length = raw_password.len();
        if (8..=20).contains(&length).not() {
            errors.push(DomainError::ValidationError {
                description: "password must be at least 8 and less than 20 characters",
            });
        }

        let has_lower_case = Regex::new("[a-z]").unwrap();
        if has_lower_case.is_match(raw_password).not() {
            errors.push(DomainError::ValidationError {
                description: "password must have lower case ascii (a-z)",
            })
        }

        let has_upper_case = Regex::new("[A-Z]").unwrap();
        if has_upper_case.is_match(raw_password).not() {
            errors.push(DomainError::ValidationError {
                description: "password must have upper case ascii (A-Z)",
            })
        }

        let has_number = Regex::new("[0-9]").unwrap();
        if has_number.is_match(raw_password).not() {
            errors.push(DomainError::ValidationError {
                description: "password must have number (0-9)",
            })
        }

        if errors.is_empty().not() {
            return Err(errors);
        }
        Ok(())
    }

    pub fn new_with(hashed_password: String) -> Self {
        Self { hashed_password }
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
    fn new_successfully() {
        let raw_password = "Password1234".to_string();

        let password = Password::new(&raw_password);

        assert!(password.is_ok());
    }

    #[test]
    fn new_with_char_errors() {
        let raw_password = "!@#$%^&*".to_string();

        let password = Password::new(&raw_password);
        let errors = password.err().unwrap();

        assert_eq!(errors.len(), 3);
    }

    #[test]
    fn new_with_non_ascii_error() {
        let raw_password = "パスワード".to_string();

        let password = Password::new(&raw_password);
        let errors = password.err().unwrap();

        assert_eq!(errors.len(), 4);
    }

    #[test]
    fn new_with_too_short_error() {
        let raw_password = "Pass1".to_string();

        let password = Password::new(&raw_password);
        let errors = password.err().unwrap();

        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn new_with_too_long_error() {
        let raw_password = "Password123456789!@#$%^&*".to_string();

        let password = Password::new(&raw_password);
        let errors = password.err().unwrap();

        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn verify_password() {
        let raw_password = "Password1234".to_string();
        let password = Password::new(&raw_password).unwrap();

        let result = password.is_same(&raw_password);

        assert!(result);
    }

    #[test]
    fn verify_invalid_password() {
        let raw_password = "Password1234".to_string();
        let invalid_password = "InvalidPassword1234".to_string();
        let password = Password::new(&raw_password).unwrap();

        let result = password.is_same(&invalid_password);

        assert!(!result)
    }
}
