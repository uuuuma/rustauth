use domain::user::value_objects::user_id::UserId;

pub trait TokenGenerator {
    fn generate(&self, user_id: &UserId) -> String;
}
