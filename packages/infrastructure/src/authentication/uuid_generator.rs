use uuid::Uuid;

use application::interfaces::authentication::id_generator::IdGenerator;

#[derive(Clone)]
pub struct UuidGenerator;

impl UuidGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for UuidGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl IdGenerator for UuidGenerator {
    fn generate(&self) -> String {
        Uuid::now_v7().to_string()
    }
}
