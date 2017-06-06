use rocket_contrib::JSON;
use validation::user::UserSerializer;
use diesel::prelude::*;
use diesel;

use models::user::{UserModel, NewUser};
use schema::users;
use schema::users::dsl::*;
use helpers::db::DB;
use responses::{APIResponse, ok, created, conflict, unauthorized};


/// Log the user in and return a response with an auth token.
///
/// Return UNAUTHORIZED in case the user can't be found or if the password is incorrect.
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

/// Register a new user using email and password.
///
/// Return CONFLICT is a user with the same email already exists.
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

    created().data(json!(&user))
}
