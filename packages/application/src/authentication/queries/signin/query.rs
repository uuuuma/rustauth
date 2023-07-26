pub struct SigninQuery {
    email: String,
    password: String,
}

impl SigninQuery {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
    pub fn email(&self) -> &String {
        &self.email
    }
    pub fn password(&self) -> &String {
        &self.password
    }
}
