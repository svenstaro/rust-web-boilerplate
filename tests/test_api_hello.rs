#![feature(use_extern_macros, proc_macro_gen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate diesel;
extern crate parking_lot;
extern crate serde_json;
extern crate uuid;
#[macro_use]
extern crate serde_derive;
extern crate speculate;

extern crate rust_web_boilerplate;

#[allow(unused_imports)]
use diesel::prelude::*;
use parking_lot::Mutex;
use rocket::http::{ContentType, Header, Status};
use rocket::local::Client;
use uuid::Uuid;
use speculate::speculate;

use factories::make_user;
use rust_web_boilerplate::rocket_factory;

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
        let (rocket, db) = rocket_factory("testing").unwrap();
        let client = Client::new(rocket).unwrap();
        #[allow(unused_variables)]
        let conn = &*db.get().expect("Failed to get a database connection for testing!");
    }

    describe "whoami" {
        it "echoes back the email" {
            let user = make_user(conn);
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
            let user = make_user(conn);
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
            let user = make_user(conn);
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
