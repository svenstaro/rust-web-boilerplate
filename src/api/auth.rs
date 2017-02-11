use rocket_contrib::JSON;
use validation::user::UserSerializer;
use frank_jwt::{Header, Payload, Algorithm, encode, decode};
use argon2rs::argon2i_simple;

use diesel::prelude::*;
use diesel;

use establish_connection;
use models::user::{UserModel, NewUser};
use schema::users;
use schema::users::dsl::*;


#[post("/login", data = "<user>", format = "application/json")]
pub fn login(user: JSON<UserSerializer>) -> String {
    let connection = establish_connection();

    let results = users.filter(email.eq(user.email.clone()))
        .first::<UserModel>(&connection)
        .expect("Error loading posts");

    // println!("Displaying {} posts", results.len());
    // for post in results {
    //     println!("{}", post.title);
    //     println!("----------\n");
    //     println!("{}", post.body);
    // }
    if user.email == "lol" && user.password == "lol" {
        let mut payload = Payload::new();
        payload.insert("user_id".to_string(), "5".to_string());
        let header = Header::new(Algorithm::HS256);
        let secret = "lolsecret";
        let jwt = encode(header, secret.to_string(), payload.clone());
        jwt
    }
    else {
        "no login".to_string()
    }
}

#[post("/register", data = "<user>", format = "application/json")]
pub fn register(user: JSON<UserSerializer>) -> String {
    let connection = establish_connection();

    let results = users.filter(email.eq(user.email.clone()))
        .first::<UserModel>(&connection);
    if results.is_ok() {
        return "conflict".to_string();
    }

    let new_user = NewUser {
        email: user.email.clone(),
        password_hash: argon2i_simple(user.password.as_str(), "login"),
    };

    diesel::insert(&new_user)
        .into(users::table)
        .execute(&connection)
        .expect("Error saving new post");
    "lol".to_string()
}
