use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FirstName(String);

impl FirstName {
    pub fn new(first_name: String) -> Self {
        Self(first_name)
    }
}

impl Display for FirstName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
