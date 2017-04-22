use rocket::testing::MockRequest;
use rocket::http::{Status, Method};
use rocket::http::ContentType;
use rust_web_boilerplate::rocket_factory;
use diesel::Connection;
use diesel::result::Error;

describe! auth_tests {
    before_each {
        let (rocket, db) = rocket_factory();
        let conn = db.get().unwrap();
        conn.begin_test_transaction().unwrap();
    }

    after_each {
        let conn = db.get().unwrap();
        Err(Error::RollbackTransaction)
    };

    describe! register {
        it "allows users to register" {
            let data = json!({
                "email": "test@example.com",
                "password": "mypassword",
            });
            let mut req = MockRequest::new(Method::Post, "/api/auth/register")
                .header(ContentType::JSON)
                .body(data.to_string());
            let mut res = req.dispatch_with(&rocket);
            let body_str = res.body().and_then(|b| b.into_string()).unwrap();

            assert_eq!(res.status(), Status::Created);
        }

        it "allows more users to register" {
            let data = json!({
                "email": "test@example.com",
                "password": "mypassword",
            });
            let mut req = MockRequest::new(Method::Post, "/api/auth/register")
                .header(ContentType::JSON)
                .body(data.to_string());
            let mut res = req.dispatch_with(&rocket);
            let body_str = res.body().and_then(|b| b.into_string()).unwrap();

            assert_eq!(res.status(), Status::Created);
        }
    }
}
