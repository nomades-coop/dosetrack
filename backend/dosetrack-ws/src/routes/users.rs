use rocket::serde::json::Json;
use rocket::State;

use crate::database;
use crate::model::dosetrack::User;
use crate::utils::GenericError;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::results::DeleteResult;
use mongodb::{bson, Cursor};
use rocket::futures::TryStreamExt;

#[get("/users")]
pub async fn get_all(database: &State<database::MongoDB>) -> Result<Json<Vec<User>>, GenericError> {
  let collection = database.collection::<User>("users");
  let mut cursor: Cursor<User> = collection.find(None, None).await.unwrap();

  let mut users: Vec<User> = Vec::new();
  while let Ok(Some(user)) = cursor.try_next().await {
    users.push(user);
  }

  Ok(Json(users))
}

#[get("/user/<id>", format = "json")]
pub async fn get(
  id: String,
  database: &State<database::MongoDB>,
) -> Result<Json<User>, Json<GenericError>> {
  dbg!(&id);
  let collection = database.collection::<User>("users");
  let user = collection
    .find_one(doc! { "_id": ObjectId::parse_str(&id).unwrap() }, None)
    .await
    .unwrap();

  match user {
    Some(u) => Ok(Json(u)),
    None => Err(Json(GenericError::new(&*format!("Usuario no encontrado")))),
  }
}

#[post("/user", format = "json", data = "<user>")]
pub async fn create_or_update(
  user: Json<User>,
  database: &State<database::MongoDB>,
) -> Result<Json<User>, GenericError> {
  let collection = database.collection::<User>("users");

  let new_user: User;

  if user._id.is_none() {
    new_user = User {
      _id: Some(bson::oid::ObjectId::new()),
      name: user.name.clone(),
      last_name: user.last_name.clone(),
      company_id: user.company_id.clone(),
      role: user.role.clone(),
      status: user.status.clone(),
    };
    let result = collection.insert_one(&new_user, None).await;
    dbg!(result);
  } else {
    new_user = User {
      _id: user._id,
      name: user.name.clone(),
      last_name: user.last_name.clone(),
      company_id: user.company_id.clone(),
      role: user.role.clone(),
      status: user.status.clone(),
    };

    let result = collection
      .replace_one(doc! { "_id":  &user._id }, &new_user, None)
      .await;
    dbg!(result);
  }
  //Ok(insert.inserted_id.to_string())

  Ok(Json(new_user))
}

#[delete("/user/<id>")]
pub async fn delete(
  id: String,
  database: &State<database::MongoDB>,
) -> Result<Json<u64>, Json<GenericError>> {
  let collection = database.collection::<User>("users");

  let result: Result<ObjectId, mongodb::bson::oid::Error> = ObjectId::parse_str(&id);
  let delete_result: Result<DeleteResult, mongodb::error::Error>;

  match result {
    Ok(oid) => {
      delete_result = collection.delete_one(doc! { "_id": oid }, None).await;
      let users_deleted = delete_result.unwrap().deleted_count;
      if users_deleted > 0 {
        Ok(Json(users_deleted))
      } else {
        Err(Json(GenericError::new(&*format!(
          "No se pudo eliminart el usuario {}",
          oid
        ))))
      }
    }
    Err(error) => {
      dbg!(&error);
      Err(Json(GenericError::new(&*format!("{:?}", error))))
    }
  }
}
