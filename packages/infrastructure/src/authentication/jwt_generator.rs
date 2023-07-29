use std::ops::Add;

use chrono::Duration;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use application::interfaces::{
    authentication::{id_generator::IdGenerator, token_generator::TokenGenerator},
    services::datetime_service::DateTimeService,
};
use domain::user::value_objects::user_id::UserId;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    exp: i64,
    iat: i64,
    nbf: i64,
    jti: String,
}

pub struct JwtGenerator<I, D>
where
    I: IdGenerator,
    D: DateTimeService,
{
    id_generator: I,
    datetime_service: D,
}

impl<I, D> JwtGenerator<I, D>
where
    I: IdGenerator,
    D: DateTimeService,
{
    pub fn new(id_generator: I, datetime_service: D) -> Self {
        Self {
            id_generator,
            datetime_service,
        }
    }
}

impl<I, D> TokenGenerator for JwtGenerator<I, D>
where
    I: IdGenerator,
    D: DateTimeService,
{
    fn generate(&self, user_id: &UserId) -> String {
        let now = self.datetime_service.jst_now();
        let exp = now.add(Duration::days(7));

        let claims = Claims {
            iss: "rustauth".to_string(),
            sub: user_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            nbf: now.timestamp(),
            jti: self.id_generator.generate(),
        };

        jsonwebtoken::encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .unwrap()
    }
}
