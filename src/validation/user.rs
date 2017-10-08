use rocket::data::{self, FromData};
use rocket::{Request, Data};
use rocket::http::Status;
use rocket::Outcome::*;
use rocket_contrib::Json;
use serde_json::Value;
use std::collections::HashMap;

use validator::Validate;
use uuid::Uuid;

#[derive(Deserialize, Debug, Validate)]
pub struct UserLogin {
    #[serde(skip_deserializing)]
    pub id: Option<Uuid>,
    #[validate(email)]
    pub email: String,
    pub password: String,
}

impl FromData for UserLogin {
    type Error = Value;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, Value> {
        let user = Json::<UserLogin>::from_data(req, data).map_failure(|_| {
            (
                Status::UnprocessableEntity,
                json!({"_schema": "Error while parsing user login."}),
            )
        })?;

        let mut errors = HashMap::new();
        if user.email == "" {
            errors.entry("email").or_insert_with(|| vec![]).push(
                "Must not be empty.",
            );
        } else if !user.email.contains('@') || !user.email.contains('.') {
            errors.entry("email").or_insert_with(|| vec![]).push(
                "Invalid email.",
            );
        }

        if user.password == "" {
            errors.entry("password").or_insert_with(|| vec![]).push(
                "Must not be empty.",
            );
        }

        if !errors.is_empty() {
            return Failure((Status::UnprocessableEntity, json!(errors)));
        }

        Success(UserLogin {
            id: user.id,
            email: user.email.clone(),
            password: user.password.clone(),
        })
    }
}
