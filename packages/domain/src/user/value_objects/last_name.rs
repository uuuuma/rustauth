use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LastName(String);

impl LastName {
    pub fn new(last_name: String) -> Self {
        Self(last_name)
    }
}

impl Display for LastName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
