use rocket::*;

#[get("/ping")]
pub fn ping() -> String {
    "PONG!".to_string()
}