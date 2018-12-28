#[allow(unused_imports)]
use diesel::prelude::*;
use parking_lot::Mutex;
use rocket::http::{ContentType, Header, Status};
use rocket::local::Client;
use rocket_contrib::json;
use serde_derive::Deserialize;
use speculate::speculate;
use uuid::Uuid;

use rust_web_boilerplate::database::DbConn;
use rust_web_boilerplate::rocket_factory;

use crate::factories::make_user;

mod common;
mod factories;

static DB_LOCK: Mutex<()> = Mutex::new(());

#[derive(Deserialize)]
struct LoginData {
    token: String,
}

speculate! {
    before {
        common::setup();
        let _lock = DB_LOCK.lock();
        let rocket = rocket_factory("testing").unwrap();
        let client = Client::new(rocket).unwrap();
        #[allow(unused_variables)]
        let conn = DbConn::get_one(client.rocket()).expect("Failed to get a database connection for testing!");
    }

    describe "whoami" {
        it "echoes back the email" {
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
            let token = body.token;

            let res = client.get("/hello/whoami")
                .header(ContentType::JSON)
                .header(Header::new("Authorization", format!("Bearer {}:{}", user.id, token)))
                .dispatch();

            assert_eq!(res.status(), Status::Ok);
        }

        it "returns BadRequest when sent no Authorization header" {
            let user = make_user(&conn);
            let data = json!({
                "email": user.email,
                "password": "testtest",
            });
            client.post("/auth/login")
                .header(ContentType::JSON)
                .body(data.to_string())
                .dispatch();

            let res = client.get("/hello/whoami")
                .header(ContentType::JSON)
                .dispatch();

            assert_eq!(res.status(), Status::BadRequest);
        }

        it "returns Unauthorized when sent an invalid token" {
            let user = make_user(&conn);
            let data = json!({
                "email": user.email,
                "password": "testtest",
            });
            client.post("/auth/login")
                .header(ContentType::JSON)
                .body(data.to_string())
                .dispatch();

            let res = client.get("/hello/whoami")
                .header(ContentType::JSON)
                .header(Header::new("Authorization", format!("Bearer {}:{}", user.id, Uuid::nil())))
                .dispatch();

            assert_eq!(res.status(), Status::Unauthorized);
        }
    }
}
