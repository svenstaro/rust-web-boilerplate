use models::user::UserModel;
use responses::{APIResponse, ok};


#[get("/whoami")]
pub fn whoami(current_user: UserModel) -> APIResponse {
    ok().data(json!(current_user.email))
}
