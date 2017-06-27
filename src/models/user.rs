use std::fmt;

use uuid::Uuid;

use chrono::{NaiveDateTime, Utc, Duration};
use argon2rs::argon2i_simple;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use ring::constant_time::verify_slices_are_equal;

use schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
#[table_name = "users"]
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

impl fmt::Display for UserModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<User {email}>", email = self.email)
    }
}

impl UserModel {
    /// Hash `password` using argon2 and return it.
    pub fn make_password_hash(password: &str) -> Vec<u8> {
        argon2i_simple(password, "loginsalt").to_vec()
    }

    /// Verify that `candidate_password` matches the stored password.
    pub fn verify_password(&self, candidate_password: &str) -> bool {
        let candidate_hash = argon2i_simple(candidate_password, "loginsalt").to_vec();
        self.password_hash == candidate_hash
    }

    /// Generate an auth token and save it to the `current_auth_token` column.
    pub fn generate_auth_token(&mut self, conn: &PgConnection) -> Result<String, DieselError> {
        let new_auth_token = Uuid::new_v4().hyphenated().to_string();
        self.current_auth_token = Some(new_auth_token.clone());
        self.last_action = Some(Utc::now().naive_utc());
        self.save_changes::<UserModel>(conn)?;
        Ok(new_auth_token)
    }

    /// Return whether or not the user has a valid auth token.
    pub fn has_valid_auth_token(&self, auth_token_timeout: Duration) -> bool {
        let latest_valid_date = Utc::now() - auth_token_timeout;
        if let Some(last_action) = self.last_action {
            if self.current_auth_token.is_some() {
                last_action > latest_valid_date.naive_utc()
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Get a `User` from a login token.
    ///
    /// A login token has this format:
    ///     <user uuid>:<auth token>
    pub fn get_user_from_login_token(token: &str, db: &PgConnection) -> Option<UserModel> {
        use schema::users::dsl::*;

        let v: Vec<&str> = token.split(':').collect();
        let (user_id, auth_token) = (Uuid::parse_str(v[0]).unwrap_or(Uuid::nil()), v[1]);

        let user = users
            .filter(id.eq(user_id))
            .first::<UserModel>(&*db)
            .optional();
        if let Ok(Some(u)) = user {
            if let Some(token) = u.current_auth_token.clone() {
                if verify_slices_are_equal(token.as_bytes(), auth_token.as_bytes()).is_ok() {
                    return Some(u);
                }
            }
        }
        return None;
    }
}
