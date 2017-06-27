use rocket::State;
use rocket_contrib::JSON;
use validation::user::UserSerializer;
use diesel::prelude::*;
use diesel;
use serde_json::Value;

use models::user::{UserModel, NewUser};
use schema::users;
use schema::users::dsl::*;
use helpers::db::DB;
use responses::{APIResponse, ok, created, conflict, unauthorized, unprocessable_entity,
                internal_server_error};
use RuntimeConfig;


/// Log the user in and return a response with an auth token.
///
/// Return UNAUTHORIZED in case the user can't be found or if the password is incorrect.
#[post("/login", data = "<user_in>", format = "application/json")]
pub fn login(
    user_in: JSON<UserSerializer>,
    db: DB,
    rconfig: State<RuntimeConfig>,
) -> Result<APIResponse, APIResponse> {
    let user_q = users
        .filter(email.eq(user_in.email.clone()))
        .first::<UserModel>(&*db)
        .optional()?;

    // For privacy reasons, we'll not provide the exact reason for failure here (although this
    // could probably be timing attacked to find out whether users exist or not.
    let mut user = user_q.ok_or(unauthorized().message(
        "Username or password incorrect.",
    ))?;
    if !user.verify_password(user_in.password.as_str()) {
        return Err(unauthorized().message("Username or password incorrect."));
    }

    let token = if user.has_valid_auth_token(rconfig.0) {
        user.current_auth_token.ok_or(internal_server_error())?
    } else {
        user.generate_auth_token(&db)?
    };

    Ok(ok().data(json!({
        "token": token
    })))
}

/// Register a new user using email and password.
///
/// Return CONFLICT is a user with the same email already exists.
#[post("/register", data = "<user>", format = "application/json")]
pub fn register(user: Result<UserSerializer, Value>, db: DB) -> Result<APIResponse, APIResponse> {
    let user_data = user.map_err(unprocessable_entity)?;
    let results = users
        .filter(email.eq(user_data.email.clone()))
        .first::<UserModel>(&*db);
    if results.is_ok() {
        return Err(conflict().message("User already exists."));
    }

    let new_password_hash = UserModel::make_password_hash(user_data.password.as_str());
    let new_user = NewUser {
        email: user_data.email.clone(),
        password_hash: new_password_hash,
    };

    let user = diesel::insert(&new_user)
        .into(users::table)
        .get_result::<UserModel>(&*db)
        .expect("Error saving new post");

    Ok(created().data(json!(&user)))
}
