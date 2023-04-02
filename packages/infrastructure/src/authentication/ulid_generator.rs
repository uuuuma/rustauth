use ulid::Ulid;

use application::interfaces::authentication::id_generator::IdGenerator;

pub struct UlidGenerator;

impl UlidGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl IdGenerator for UlidGenerator {
    fn generate(&self) -> String {
        Ulid::new().to_string()
    }
}
