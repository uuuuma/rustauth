use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Self {
        Self(email)
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
