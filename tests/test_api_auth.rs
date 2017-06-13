use diesel::prelude::*;
use parking_lot::Mutex;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;
use uuid::Uuid;
use serde_json;
use serde_json::Value;

use rust_web_boilerplate::rocket_factory;
use rust_web_boilerplate::models::user::UserModel;
use rust_web_boilerplate::schema::users;
use rust_web_boilerplate::schema::users::dsl::*;

use factories::make_user;

static DB_LOCK: Mutex<()> = Mutex::new(());

#[derive(Deserialize)]
struct LoginData {
    token: String,
}

describe! auth_tests {
    before_each {
        let _lock = DB_LOCK.lock();
        let (rocket, db) = rocket_factory();
        let client = Client::new(rocket).unwrap();
        let conn = &*db.get().expect("Failed to get a database connection for testing!");
    }

    describe! login {
        it "enables users to login and get back a valid auth token" {
            let user = make_user(&conn);
            let data = json!({
                "email": user.email,
                "password": "testtest",
            });
            let mut res = client.post("/auth/login")
                .header(ContentType::JSON)
                .body(data.to_string())
                .dispatch();
            let body: LoginData = serde_json::from_str(&res.body_string().unwrap()).unwrap();

            let refreshed_user = users
                .find(user.id)
                .first::<UserModel>(conn).unwrap();
            assert_eq!(res.status(), Status::Ok);
            assert_eq!(body.token, refreshed_user.current_auth_token.unwrap());
        }

        it "can log in and get back the same auth token if there's already a valid one" {
            let user = make_user(&conn);
            let data = json!({
                "email": user.email,
                "password": "testtest",
            });

            // Login the first time and then retrieve and store the token.
            let first_login_token = {
                client.post("/auth/login")
                    .header(ContentType::JSON)
                    .body(data.to_string())
                    .dispatch();
                let user_after_first_login = users
                    .find(user.id)
                    .first::<UserModel>(conn).unwrap();
                user_after_first_login.current_auth_token.unwrap()
            };

            // Login the second time and then retrieve and store the token.
            let second_login_token = {
                client.post("/auth/login")
                    .header(ContentType::JSON)
                    .body(data.to_string())
                    .dispatch();
                let user_after_second_login = users
                    .find(user.id)
                    .first::<UserModel>(conn).unwrap();
                user_after_second_login.current_auth_token.unwrap()
            };

            assert_eq!(first_login_token, second_login_token);
        }

        it "fails with a wrong username" {
            make_user(&conn);
            let data = json!({
                    "email": "invalid@example.com",
                    "password": "testtest",
            });
            let mut res = client.post("/auth/login")
                .header(ContentType::JSON)
                .body(data.to_string())
                .dispatch();
            let body: Value = serde_json::from_str(&res.body_string().unwrap()).unwrap();

            assert_eq!(res.status(), Status::Unauthorized);
            assert_eq!(body["message"], "Username or password incorrect.");
        }

        it "fails with a wrong password" {
            let user = make_user(&conn);
            let data = json!({
                    "email": user.email,
                    "password": "invalid",
            });
            let mut res = client.post("/auth/login")
                .header(ContentType::JSON)
                .body(data.to_string())
                .dispatch();
            let body: Value = serde_json::from_str(&res.body_string().unwrap()).unwrap();

            assert_eq!(res.status(), Status::Unauthorized);
            assert_eq!(body["message"], "Username or password incorrect.");
        }
    }

    describe! register {
        it "allows users to register a new account and then login with it" {
            let new_email = format!("{username}@example.com", username=Uuid::new_v4().hyphenated().to_string());
            let new_password = "mypassword";
            let data = json!({
                "email": new_email,
                "password": new_password,
            });
            let mut res = client.post("/auth/register")
                .header(ContentType::JSON)
                .body(data.to_string())
                .dispatch();
            let body: UserModel = serde_json::from_str(&res.body_string().unwrap()).unwrap();

            assert_eq!(res.status(), Status::Created);
            assert_eq!(body.email, new_email);

            // Now try to log in using the new account.
            let data = json!({
                "email": new_email,
                "password": new_password,
            });
            let mut res = client.post("/auth/login")
                .header(ContentType::JSON)
                .body(data.to_string())
                .dispatch();
            let body: LoginData = serde_json::from_str(&res.body_string().unwrap()).unwrap();

            let logged_in_user = users
                .filter(email.eq(new_email))
                .first::<UserModel>(conn).unwrap();
            assert_eq!(res.status(), Status::Ok);
            assert_eq!(body.token, logged_in_user.current_auth_token.unwrap());
        }

        it "can't register with an existing email" {
            let new_email = format!("{username}@example.com", username=Uuid::new_v4().hyphenated().to_string());
            let new_password = "mypassword";
            let data = json!({
                "email": new_email,
                "password": new_password,
            });
            let mut res = client.post("/auth/register")
                .header(ContentType::JSON)
                .body(data.to_string())
                .dispatch();

            let mut res = client.post("/auth/register")
                .header(ContentType::JSON)
                .body(data.to_string())
                .dispatch();

            assert_eq!(res.status(), Status::Conflict);
        }

        it "can't register with an invalid email" {
            let data = json!({
                "email": "invalid",
                "password": "somepw",
            });
            let mut res = client.post("/auth/register")
                .header(ContentType::JSON)
                .body(data.to_string())
                .dispatch();

            assert_eq!(res.status(), Status::UnprocessableEntity);
        }

        it "can't register with an empty email" {
            let data = json!({
                "email": "",
                "password": "somepw",
            });
            let mut res = client.post("/auth/register")
                .header(ContentType::JSON)
                .body(data.to_string())
                .dispatch();

            assert_eq!(res.status(), Status::UnprocessableEntity);
        }

        it "can't register with an empty password" {
            let data = json!({
                "email": "something@example.com",
                "password": "",
            });
            let mut res = client.post("/auth/register")
                .header(ContentType::JSON)
                .body(data.to_string())
                .dispatch();

            assert_eq!(res.status(), Status::UnprocessableEntity);
        }
    }
}
