use rocket::serde::json::Json;
use rocket::State;

use mongodb::bson::{doc, oid::ObjectId};
use mongodb::results::DeleteResult;
use mongodb::{bson, Cursor};

use crate::database;
use crate::model::dosetrack::{Operator, OperatorStatus};
use crate::utils::GenericError;
use rocket::futures::TryStreamExt;

#[get("/operators")]
pub async fn get_all(
  database: &State<database::MongoDB>,
) -> Result<Json<Vec<Operator>>, GenericError> {
  let collection = database.collection::<Operator>("operators");
  let mut cursor: Cursor<Operator> = collection.find(None, None).await.unwrap();

  let mut operators: Vec<Operator> = Vec::new();
  while let Ok(Some(operator)) = cursor.try_next().await {
    operators.push(operator);
  }

  Ok(Json(operators))
}

#[get("/operator/<id>", format = "json")]
pub async fn get(
  id: String,
  database: &State<database::MongoDB>,
) -> Result<Json<Operator>, Json<GenericError>> {
  dbg!(&id);
  let collection = database.collection::<Operator>("operators");
  let operator = collection
    .find_one(doc! { "_id": ObjectId::parse_str(&id).unwrap() }, None)
    .await
    .unwrap();

  match operator {
    Some(o) => Ok(Json(o)),
    None => Err(Json(GenericError::new(&*format!("Usuario no encontrado")))),
  }
}
