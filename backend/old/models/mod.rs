use rocket::form::{FromForm, FromFormField};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, FromForm, Deserialize, Clone)]
pub struct BlogEntry {
  // using String (instead of bson::oid::ObjectId) so it can easily be used as
  // a FromFormField without creating a custom transformer like (https://github.com/SergioBenitez/Rocket/issues/602)
  pub _id: String,
  #[field(validate = len(2..))]
  pub title: String,
  #[field(validate = len(10..))]
  pub content: String,
  #[field(validate = len(2..))]
  pub author: String,
  #[field(default = "Today")]
  pub last_edit_date: String,
  pub status: Status,
}

#[derive(Debug, FromFormField, Serialize, Deserialize, Clone)]
pub enum Status {
  Draft,
  Published,
}
