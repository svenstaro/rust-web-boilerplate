use rocket::data::{self, FromData, FromDataSimple, Transform};
use rocket::http::Status;
use rocket::Outcome::*;
use rocket::{Data, Request};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::io::Read;
use uuid::Uuid;
use validator::Validate;
use validator_derive::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct UserLogin {
    #[serde(skip_deserializing)]
    pub id: Option<Uuid>,
    #[validate(email)]
    pub email: String,
    pub password: String,
}

impl FromDataSimple for UserLogin {
    type Error = JsonValue;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, JsonValue> {
        let mut d = String::new();
        if data.open().read_to_string(&mut d).is_err() {
            return Failure((
                Status::InternalServerError,
                json!({"_schema": "Internal server error."}),
            ));
        }
        let user =
            Json::<UserLogin>::from_data(req, Transform::Owned(Success(d))).map_failure(|_| {
                (
                    Status::UnprocessableEntity,
                    json!({"_schema": "Error while parsing user login."}),
                )
            })?;

        let mut errors = HashMap::new();
        if user.email == "" {
            errors
                .entry("email")
                .or_insert_with(|| vec![])
                .push("Must not be empty.");
        } else if !user.email.contains('@') || !user.email.contains('.') {
            errors
                .entry("email")
                .or_insert_with(|| vec![])
                .push("Invalid email.");
        }

        if user.password == "" {
            errors
                .entry("password")
                .or_insert_with(|| vec![])
                .push("Must not be empty.");
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
