use crate::database::MongoDB;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::*;

#[derive(Serialize, Deserialize)]
pub struct Response {
  status: String,
  message: String,
}
impl Response {
  fn ok(msg: &str) -> Self {
    Response {
      status: "Success".to_string(),
      message: msg.to_string(),
    }
  }

  fn error(msg: &str) -> Self {
    Response {
      status: "Error".to_string(),
      message: msg.to_string(),
    }
  }
}

#[get("/users")]
pub fn list_users() -> Json<Response> {
  Json(Response::ok("List of users"))
}

#[post("/users")]
pub fn create_user() -> Json<Response> {
  Json(Response::ok("Creation of user"))
}

#[get("/user/<id>")]
pub fn get_user(id: String) -> Json<Response> {
  Json(Response::ok(&*format!("Info for user {}", id)))
}

#[put("/user/<id>")]
pub fn update_user(id: String) -> Json<Response> {
  Json(Response::ok(&*format!("Update user {}", id)))
}

#[delete("/user/<id>")]
pub fn delete_user(id: String) -> Json<Response> {
  Json(Response::ok(&*format!("User {} deleted!", id)))
}
