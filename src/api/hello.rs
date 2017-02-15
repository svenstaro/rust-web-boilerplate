use models::user::UserModel;
use helpers::db::DB;
use responses::{APIResponse, ok};


#[get("/whoami")]
pub fn whoami(current_user: UserModel, db: DB) -> APIResponse {
    ok().data(json!(current_user.email))
}
