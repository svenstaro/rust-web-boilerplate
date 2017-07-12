use rocket::data::{self, FromData};
use rocket::{Request, Data};
use rocket::http::Status;
use rocket::Outcome::*;
use rocket_contrib::Json;
use serde_json::Value;
use std::collections::HashMap;

use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UserSerializer {
    #[serde(skip_deserializing)]
    pub id: Option<Uuid>,
    #[validate(email)]
    pub email: String,
    pub password: String,
}

impl FromData for UserSerializer {
    type Error = Value;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, Value> {
        let user = Json::<UserSerializer>::from_data(req, data).map_failure(
            |_| (Status::UnprocessableEntity, json!({"_schema": "Error while serialzing."})),
        )?;

        let mut errors = HashMap::new();
        if user.email == "" {
            errors.entry("email").or_insert(vec![]).push(
                "Must not be empty.",
            );
        } else if !user.email.contains("@") || !user.email.contains(".") {
            errors.entry("email").or_insert(vec![]).push(
                "Invalid email.",
            );
        }

        if user.password == "" {
            errors.entry("password").or_insert(vec![]).push(
                "Must not be empty.",
            );
        }

        if !errors.is_empty() {
            return Failure((Status::UnprocessableEntity, json!(errors)));
        }

        return Success(UserSerializer {
            id: None,
            email: user.email.clone(),
            password: user.password.clone(),
        });
    }
}
