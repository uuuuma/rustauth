use std::{fmt::Display, ops::Not};

use regex::Regex;

use crate::error::DomainError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Email(String);

impl Email {
    pub fn new(email: &String) -> Result<Self, Vec<DomainError>> {
        Self::validate(email)?;

        Ok(Self(email.to_string()))
    }

    fn validate(email: &str) -> Result<(), Vec<DomainError>> {
        let mut errors = vec![];

        let email_regex = Regex::new(
            r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$",
        ).unwrap();

        if email_regex.is_match(email).not() {
            errors.push(DomainError::ValidationError {
                description: "email format is invalid. see https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/email#validation",
            });
        }

        if errors.is_empty().not() {
            return Err(errors);
        }
        Ok(())
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_successfully() {
        let email = "user+test@example.com".to_string();

        let result = Email::new(&email);

        assert!(result.is_ok());
    }

    #[test]
    fn new_with_format_error() {
        let email = "user-email".to_string();

        let result = Email::new(&email);

        assert!(result.is_err());
    }
}
