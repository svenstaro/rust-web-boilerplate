use models::user::UserModel;
use helpers::db::DB;


#[get("/whoami")]
pub fn whoami(user: UserModel, db: DB) -> String {
    user.email
}
