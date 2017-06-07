use parking_lot::Mutex;
use rocket::http::{Status, Method};
use rust_web_boilerplate::rocket_factory;
use uuid::Uuid;
use diesel::prelude::*;

use rust_web_boilerplate::models::user::UserModel;
use rust_web_boilerplate::schema::users;
use rust_web_boilerplate::schema::users::dsl::*;

use factories::make_user;
use utils::{test_request_post, parse_body_json};

static DB_LOCK: Mutex<()> = Mutex::new(());

describe! auth_tests {
    before_each {
        let _lock = DB_LOCK.lock();
        let (rocket, db) = rocket_factory();
        let conn = &*db.get().expect("Failed to get a database connection for testing!");
    }

    // after_each {
    // }

    describe! login {
        it "enables users to login and get back a valid auth token" {
            let user = make_user(&conn);
            let data = json!({
                "email": user.email,
                "password": "testtest",
            });
            let mut res = test_request_post("/auth/login", &data, &rocket);
            let body = parse_body_json(&mut res);

            let refreshed_user = users
                .find(user.id)
                .first::<UserModel>(conn).unwrap();
            assert_eq!(res.status(), Status::Ok);
            assert_eq!(body["token"], refreshed_user.current_auth_token.unwrap());
        }

        it "can log in and get back the same auth token if there's already a valid one" {
            let user = make_user(&conn);
            let data = json!({
                "email": user.email,
                "password": "testtest",
            });

            // Login the first time and then retrieve and store the token.
            let first_login_token = {
                let mut res = test_request_post("/auth/login", &data, &rocket);
                let user_after_first_login = users
                    .find(user.id)
                    .first::<UserModel>(conn).unwrap();
                user_after_first_login.current_auth_token.unwrap()
            };

            // Login the second time and then retrieve and store the token.
            let second_login_token = {
                let mut res = test_request_post("/auth/login", &data, &rocket);
                let user_after_second_login = users
                    .find(user.id)
                    .first::<UserModel>(conn).unwrap();
                user_after_second_login.current_auth_token.unwrap()
            };

            assert_eq!(first_login_token, second_login_token);
        }

        // it "fails with a wrong username" {
        //     let user = make_user(&conn);
        //     let mut req = MockRequest::new(Method::Post, "/auth/login")
        //         .header(ContentType::JSON)
        //         .body(json!({
        //             "email": user.email,
        //             "password": "invalid",
        //         }).to_string());
        //     let mut res = req.dispatch_with(&rocket);
        //     let body = parse_body_json(&mut res);
        //
        //     let refreshed_user = users
        //         .find(user.id)
        //         .first::<UserModel>(conn).unwrap();
        //     assert_eq!(res.status(), Status::Ok);
        //     assert_eq!(body["token"], refreshed_user.current_auth_token.unwrap());
        // }
    }

    describe! register {
        it "allows users to register a new account and then login with it" {
            let new_email = format!("{username}@example.com", username=Uuid::new_v4().hyphenated().to_string());
            let data = json!({
                "email": new_email,
                "password": "mypassword",
            });
            let mut res = test_request_post("/auth/register", &data, &rocket);
            let _body_str = res.body().and_then(|b| b.into_string()).unwrap();

            assert_eq!(res.status(), Status::Created);
        }
    }
}
