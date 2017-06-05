use uuid::Uuid;

use chrono::NaiveDateTime;
use argon2rs::argon2i_simple;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use ring::constant_time::verify_slices_are_equal;

use schema::users;
use helpers::db::DB;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct UserModel {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub email: String,
    pub password_hash: Vec<u8>,
    pub current_auth_token: Option<String>,
    pub last_action: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub email: String,
    pub password_hash: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserLoginToken {
    user_id: Uuid,
}

impl UserModel {

    pub fn make_password_hash(new_password: &str) -> Vec<u8> {
        argon2i_simple(new_password, "loginsalt").to_vec()
    }

    pub fn verify_password(&self, candidate_password: &str) -> bool {
        let candidate_hash = argon2i_simple(candidate_password, "loginsalt").to_vec();
        self.password_hash == candidate_hash
    }

    pub fn generate_auth_token(&self, salt: &str) -> String {
        "TODO".to_owned()
    }

    /// Get a `User` from a login token.
    ///
    /// A login token has this format:
    ///     <user uuid>:<auth token>
    pub fn get_user_from_login_token(token: &str, db: &PgConnection) -> Option<UserModel> {
        use schema::users::dsl::*;

        // let (user_id, auth_token) = token.split(':').collect();

        // let user = users.filter(id.eq(user_id)).first::<UserModel>(&*db).optional();
        // if let Some(user) = user {
        //     if user.current_auth_token {
        //         if verify_slices_are_equal(user.current_auth_token, auth_token).is_ok() {
        //             return Some(user.unwrap());
        //         }
        //     }
        // }
        return None;
    }
}
