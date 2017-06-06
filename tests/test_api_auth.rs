use parking_lot::Mutex;
use rocket::testing::MockRequest;
use rocket::http::{Status, Method};
use rocket::http::ContentType;
use rust_web_boilerplate::rocket_factory;
use uuid::Uuid;

use factories::make_user;
use utils::parse_body_json;
// use diesel::prelude::*;
// use diesel::result::Error;

static DB_LOCK: Mutex<()> = Mutex::new(());

describe! auth_tests {
    before_each {
        let _lock = DB_LOCK.lock();
        let (rocket, db) = rocket_factory();
        let conn = db.get().expect("Failed to get a database connection for testing!");
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
            let mut req = MockRequest::new(Method::Post, "/auth/login")
                .header(ContentType::JSON)
                .body(data.to_string());
            let mut res = req.dispatch_with(&rocket);
            let body = parse_body_json(&mut res);

            assert_eq!(res.status(), Status::Ok);
            // assert_eq!(body, "");
        }
    }

    describe! register {
        it "allows users to register a new account and then login with it" {
            let new_email = format!("{username}@example.com", username=Uuid::new_v4().hyphenated().to_string());
            let data = json!({
                "email": new_email,
                "password": "mypassword",
            });
            let mut req = MockRequest::new(Method::Post, "/auth/register")
                .header(ContentType::JSON)
                .body(data.to_string());
            let mut res = req.dispatch_with(&rocket);
            let _body_str = res.body().and_then(|b| b.into_string()).unwrap();

            assert_eq!(res.status(), Status::Created);
        }
    }
}
