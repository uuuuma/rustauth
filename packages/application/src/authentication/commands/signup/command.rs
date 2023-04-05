pub struct SignupCommand {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

impl SignupCommand {
    pub fn new(first_name: String, last_name: String, email: String, password: String) -> Self {
        Self {
            first_name,
            last_name,
            email,
            password,
        }
    }
    pub fn first_name(&self) -> &String {
        &self.first_name
    }
    pub fn last_name(&self) -> &String {
        &self.last_name
    }
    pub fn email(&self) -> &String {
        &self.email
    }
    pub fn password(&self) -> &String {
        &self.password
    }
}
