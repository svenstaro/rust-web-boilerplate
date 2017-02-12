use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket_contrib::{JSON, Value};
use r2d2::GetTimeout;

use models::user::UserModel;
use helpers::db::{DB_POOL, DB};


#[error(400)]
fn bad_request() -> JSON<Value> {
    JSON(json!({
        "status": "error",
        "reason": "Bad request."
    }))
}

#[error(401)]
fn unauthorized() -> JSON<Value> {
    JSON(json!({
        "status": "error",
        "reason": "Unauthorized."
    }))
}

#[error(403)]
fn forbidden() -> JSON<Value> {
    JSON(json!({
        "status": "error",
        "reason": "Forbidden."
    }))
}

#[error(404)]
fn not_found() -> JSON<Value> {
    JSON(json!({
        "status": "error",
        "reason": "Not found."
    }))
}

impl<'a, 'r> FromRequest<'a, 'r> for UserModel {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserModel, ()> {
        let tokens: Vec<_> = request.headers().get("Authorization").collect();
        if tokens.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let token = tokens[0];

        if let Some(user) = UserModel::get_user_from_auth_token(&token, "loginsalt") {
            return Outcome::Success(user);
        }

        return Outcome::Failure((Status::Unauthorized, ()));
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
	type Error = GetTimeout;
	fn from_request(_: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
		match DB_POOL.get() {
			Ok(conn) => Outcome::Success(DB(conn)),
			Err(e) => Outcome::Failure((Status::InternalServerError, e)),
		}
	}
}
