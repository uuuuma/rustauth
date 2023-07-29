#[derive(Debug)]
pub struct SigninResponse {
    token: String,
}

impl SigninResponse {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}
