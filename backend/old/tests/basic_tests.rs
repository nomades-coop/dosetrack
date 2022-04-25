use rocket::http::{ContentType, Status};

mod common;

#[test]
fn ping_test() {
  // let rocket = rocket::build();
  let client = common::setup();
  let response = client.get("/ping").dispatch();
  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.into_string(), Some("PONG!".into()));
}

#[test]
fn get_user_list_test() {
  let client = common::setup();
  let response = client.get("/api/users").dispatch();
  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.content_type(), Some(ContentType::JSON));
  assert_eq!(
    response.into_string(),
    Some(r#"{"status":"Success","message":"List of users"}"#.to_string())
  )
}

#[test]
fn new_user_test() {
  let client = common::setup();
  let response = client.post("/api/users").dispatch();
  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.content_type(), Some(ContentType::JSON));
  assert_eq!(
    response.into_string(),
    Some("{\"status\":\"Success\",\"message\":\"Creation of user\"}".into())
  );
}

#[test]
fn info_user_test() {
  let client = common::setup();
  let response = client.get("/api/user/1").dispatch();
  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.content_type(), Some(ContentType::JSON));
  assert_eq!(
    response.into_string(),
    Some("{\"status\":\"Success\",\"message\":\"Info for user 1\"}".into())
  );
}

#[test]
fn update_user_test() {
  let client = common::setup();
  let response = client.put("/api/user/1").dispatch();
  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.content_type(), Some(ContentType::JSON));
  assert_eq!(
    response.into_string(),
    Some("{\"status\":\"Success\",\"message\":\"Update user 1\"}".into())
  );
}

#[test]
fn delete_user_test() {
  let client = common::setup();
  let response = client.delete("/api/user/1").dispatch();
  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.content_type(), Some(ContentType::JSON));
  assert_eq!(
    response.into_string(),
    Some("{\"status\":\"Success\",\"message\":\"User 1 deleted!\"}".into())
  );
}
