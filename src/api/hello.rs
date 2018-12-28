use rocket::get;
use rocket_contrib::json;

use crate::models::user::UserModel;
use crate::responses::{ok, APIResponse};

#[get("/whoami")]
pub fn whoami(current_user: UserModel) -> APIResponse {
    ok().data(json!(current_user.email))
}
