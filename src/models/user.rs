use uuid::Uuid;

use chrono::NaiveDateTime;
use jsonwebtoken::{encode, decode, Header, Algorithm};
use argon2rs::argon2i_simple;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use schema::users;
use helpers::db::DB;
use helpers::util;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct UserModel {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub email: String,
    pub password_hash: Vec<u8>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
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
        let secret = util::get_secret();

        // TODO: This is probably not a good way to do that.
        let combined_secret = secret + salt;

        encode(Header::default(),
               &UserLoginToken { user_id: self.id },
               combined_secret.as_bytes())
                .unwrap()
    }

    pub fn get_user_from_auth_token(token: &str, salt: &str, db: &PgConnection) -> Option<UserModel> {
        use schema::users::dsl::*;

        let secret = util::get_secret();

        // TODO: This is probably not a good way to do that.
        let combined_secret = secret + salt;

        let decrypted_token =
            decode::<UserLoginToken>(&token, combined_secret.as_bytes(), Algorithm::HS256);
        if decrypted_token.is_err() {
            return None;
        }

        let token = decrypted_token.unwrap();

        let user = users.filter(id.eq(token.claims.user_id))
            .first::<UserModel>(&*db);
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
    pub password_hash: Vec<u8>,
}
