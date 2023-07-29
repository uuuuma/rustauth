use ulid::Ulid;

use application::interfaces::authentication::id_generator::IdGenerator;

#[derive(Clone)]
pub struct UlidGenerator;

impl UlidGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for UlidGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl IdGenerator for UlidGenerator {
    fn generate(&self) -> String {
        Ulid::new().to_string()
    }
}
