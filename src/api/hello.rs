use models::user::UserModel;

#[get("/whoami")]
pub fn whoami(user: UserModel) -> String {
    user.email
}
