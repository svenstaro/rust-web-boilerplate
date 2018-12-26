use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

use crate::responses::{
    bad_request, forbidden, internal_server_error, not_found, service_unavailable, unauthorized,
    APIResponse,
};
use crate::models::user::UserModel;

#[catch(400)]
pub fn bad_request_handler() -> APIResponse {
    bad_request()
}

#[catch(401)]
pub fn unauthorized_handler() -> APIResponse {
    unauthorized()
}

#[catch(403)]
pub fn forbidden_handler() -> APIResponse {
    forbidden()
}

#[catch(404)]
pub fn not_found_handler() -> APIResponse {
    not_found()
}

#[catch(500)]
pub fn internal_server_error_handler() -> APIResponse {
    internal_server_error()
}

#[catch(503)]
pub fn service_unavailable_handler() -> APIResponse {
    service_unavailable()
}

// impl<'a, 'r> FromRequest<'a, 'r> for UserModel {
//     type Error = ();
//
//     fn from_request(request: &'a Request<'r>) -> request::Outcome<UserModel, ()> {
//         let db = <DB as FromRequest>::from_request(request)?;
//         let keys: Vec<_> = request.headers().get("Authorization").collect();
//         if keys.len() != 1 {
//             return Outcome::Failure((Status::BadRequest, ()));
//         };
//
//         let token_header = keys[0];
//         let token = token_header.replace("Bearer ", "");
//
//         match UserModel::get_user_from_login_token(&token, &*db) {
//             Some(user) => Outcome::Success(user),
//             None => Outcome::Failure((Status::Unauthorized, ())),
//         }
//     }
// }
