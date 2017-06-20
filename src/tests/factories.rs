use uuid::Uuid;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use models::user::{UserModel, NewUser};
use schema::users;

/// Create a new `User` and add it to the database.
///
/// The user's email will be set to '<uuid>@example.com'.
pub fn make_user(conn: &PgConnection) -> UserModel {
    let new_email = format!("{username}@example.com", username=Uuid::new_v4().hyphenated().to_string());
    let new_password_hash = UserModel::make_password_hash("testtest");
    let new_user = NewUser {
        email: new_email,
        password_hash: new_password_hash,
    };

    let user = diesel::insert(&new_user)
        .into(users::table)
        .get_result::<UserModel>(conn)
        .expect("Error saving new post");
    user
}
