use rocket::State;
use rocket_contrib::JSON;
use validation::user::UserSerializer;
use diesel::prelude::*;
use diesel;

use models::user::{UserModel, NewUser};
use schema::users;
use schema::users::dsl::*;
use helpers::db::DB;
use responses::{APIResponse, ok, created, conflict, internal_server_error};
use RuntimeConfig;


/// Log the user in and return a response with an auth token.
///
/// Return UNAUTHORIZED in case the user can't be found or if the password is incorrect.
#[post("/login", data = "<user_in>", format = "application/json")]
pub fn login(user_in: JSON<UserSerializer>,
             db: DB,
             rconfig: State<RuntimeConfig>)
             -> Result<APIResponse, APIResponse> {
    let user_q = users
        .filter(email.eq(user_in.email.clone()))
        .first::<UserModel>(&*db)
        .optional()
        .or(Err(internal_server_error()))?;

    // For privacy reasons, we'll not provide the exact reason for failure here.
    let mut user = user_q
        .ok_or(internal_server_error().message("Username or password incorrect."))?;
    if !user.verify_password(user_in.password.as_str()) {
        return Err(internal_server_error().message("Username or password incorrect."));
    }

    let token = if user.has_valid_auth_token(rconfig.0) {
        user.current_auth_token.ok_or(internal_server_error())?
    } else {
        user.generate_auth_token(&db)
    };

    Ok(ok().data(json!({
                           "token": token
                       })))
}

/// Register a new user using email and password.
///
/// Return CONFLICT is a user with the same email already exists.
#[post("/register", data = "<user>", format = "application/json")]
pub fn register(user: JSON<UserSerializer>, db: DB) -> APIResponse {
    let results = users
        .filter(email.eq(user.email.clone()))
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
