use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use r2d2::GetTimeout;

use models::user::UserModel;
use helpers::db::{DB_POOL, DB};
use responses::{APIResponse, bad_request, unauthorized, forbidden, not_found};


#[error(400)]
fn bad_request_handler() -> APIResponse {
    bad_request()
}

#[error(401)]
fn unauthorized_handler() -> APIResponse {
    unauthorized()
}

#[error(403)]
fn forbidden_handler() -> APIResponse {
    forbidden()
}

#[error(404)]
fn not_found_handler() -> APIResponse {
    not_found()
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
