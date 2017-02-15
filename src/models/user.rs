use uuid::Uuid;
use chrono::NaiveDateTime;
use jsonwebtoken::{encode, decode, Header, Algorithm};
use argon2rs::argon2i_simple;
use diesel::prelude::*;

use schema::users;
use helpers::db::DB_POOL;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct UserModel {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct UserLoginToken {
    user_id: Uuid,
}

impl UserModel {
    pub fn make_password_hash(new_password: &str) -> String {
        let password_hash = argon2i_simple(new_password, "loginsalt");
        String::from_utf8_lossy(&password_hash).into_owned()
    }

    pub fn verify_password(&self, candidate_password: &str) -> bool {
        let candidate_password = argon2i_simple(candidate_password, "loginsalt");
        let candidate_password_string = String::from_utf8_lossy(&candidate_password);
        self.password_hash == candidate_password_string
    }

    pub fn generate_auth_token(&self, salt: &str) -> String {
        // TODO: Fetch secret from config.
        let secret = String::from("lolsecret");

        // TODO: This is probably not a good way to do that.
        let combined_secret = secret + salt;

        encode(Header::default(),
               &UserLoginToken { user_id: self.id },
               combined_secret.as_bytes())
            .unwrap()
    }

    pub fn get_user_from_auth_token(token: &str, salt: &str) -> Option<UserModel> {
        use schema::users::dsl::*;

        // TODO: Fetch secret from config.
        let secret = String::from("lolsecret");

        // TODO: This is probably not a good way to do that.
        let combined_secret = secret + salt;

        let decrypted_token =
            decode::<UserLoginToken>(&token, combined_secret.as_bytes(), Algorithm::HS256);
        if decrypted_token.is_err() {
            return None;
        }

        let token = decrypted_token.unwrap();

        let connection = DB_POOL.get().unwrap();
        let user = users.filter(id.eq(token.claims.user_id))
            .first::<UserModel>(&*connection);
        if user.is_err() {
            return None;
        }

        Some(user.unwrap())
    }
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
}
