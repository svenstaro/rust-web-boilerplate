use rocket_contrib::{JSON, Value};
use validation::user::UserSerializer;
use diesel::prelude::*;
use diesel;
use diesel::pg::PgConnection;

use models::user::{UserModel, NewUser};
use schema::users;
use schema::users::dsl::*;
use helpers::db::DB;
use responses::{APIResponse, ok, created, conflict, unauthorized};


#[post("/login", data = "<user_in>", format = "application/json")]
pub fn login(user_in: JSON<UserSerializer>, db: DB) -> APIResponse {
    let results = users.filter(email.eq(user_in.email.clone()))
        .first::<UserModel>(&*db);

    if results.is_err() {
        return unauthorized().message("Username or password incorrect.");
    }

    let user = results.unwrap();
    if !user.verify_password(user_in.password.as_str()) {
        return unauthorized().message("Username or password incorrect.");
    }

    ok().data(json!(user.generate_auth_token("loginsalt")))
}

#[post("/register", data = "<user>", format = "application/json")]
pub fn register(user: JSON<UserSerializer>, db: DB) -> APIResponse {
    let results = users.filter(email.eq(user.email.clone()))
        .first::<UserModel>(&*db);
    if results.is_ok() {
        return conflict().message("User already exists.");
    }

    let new_password_hash = UserModel::make_password_hash(user.password.as_str());
    let new_user = NewUser {
        email: user.email.clone(),
        password_hash: new_password_hash,
    };

    let user = diesel::insert(&new_user)
        .into(users::table)
        .get_result::<UserModel>(&*db)
        .expect("Error saving new post");

    created().message("User created.").data(json!(&user))
}
