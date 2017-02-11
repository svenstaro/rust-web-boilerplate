#[get("/whoami")]
pub fn whoami() -> &'static str {
    "lol"
}
