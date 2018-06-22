extern crate dotenv;

pub fn setup() {
    dotenv::dotenv().ok();
}
