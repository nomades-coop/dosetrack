use rocket::serde::json::Json;
use rocket::State;
use std::ops::Deref;

use mongodb::{
  bson,
  bson::{doc, oid::ObjectId, DateTime},
  options::{Collation, FindOptions},
  results::DeleteResult,
  Cursor,
};

use crate::database;
use crate::model::dosetrack::Operator;
use crate::utils::GenericError;
use rocket::futures::TryStreamExt;

#[get("/operators/<company_id>")]
pub async fn get_by_company(
  company_id: String,
  database: &State<database::MongoDB>,
) -> Result<Json<Vec<Operator>>, GenericError> {
  let collection = database.collection::<Operator>("operators");

  let col = Collation::builder().locale("es").build();

  let filter = doc! {
   "company_id": ObjectId::parse_str(&company_id).unwrap(),
  };
  let options = FindOptions::builder()
    .projection(doc! {
      "_id": 1,
      "company_id": 1,
      "name": 1,
      "dni": 1,
      "licence": 1,
      "dosimeter_id": 1,
      "status":1
    })
    .collation(col)
    .sort(doc! {
      "name": 1i32
    })
    .build();

  let mut cursor: Cursor<Operator> = collection.find(filter, options).await.unwrap();

  let mut operators: Vec<Operator> = Vec::new();
  while let Ok(Some(operator)) = cursor.try_next().await {
    operators.push(operator);
  }

  Ok(Json(operators))
}

#[get("/operators/overdose")]
pub async fn overdose(
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
  let collection = database.collection::<Operator>("operators");
  let operator = collection
    .find_one(doc! { "_id": ObjectId::parse_str(&id).unwrap() }, None)
    .await
    .unwrap();

  match operator {
    Some(o) => Ok(Json(o)),
    None => Err(Json(GenericError::new(&*format!(
      "Operador {} no encontrado",
      &id
    )))),
  }
}

// TODO: Modificar el resto de las rutas segun este modelo para no usar clone en las estructuras
#[post("/operator", format = "json", data = "<new_operator>")]
pub async fn create_or_update(
  mut new_operator: Json<Operator>,
  database: &State<database::MongoDB>,
) -> Result<Json<Operator>, GenericError> {
  let collection = database.collection::<Operator>("operators");

  if new_operator._id.is_none() {
    new_operator._id = Some(bson::oid::ObjectId::new());
    let _result = collection.insert_one(new_operator.deref(), None).await;
  } else {
    let _result = collection
      .replace_one(
        doc! { "_id":  &new_operator._id },
        new_operator.deref(),
        None,
      )
      .await;
  }

  Ok(Json(new_operator.into_inner()))
}

#[delete("/operator/<id>")]
pub async fn delete(
  id: String,
  database: &State<database::MongoDB>,
) -> Result<Json<u64>, Json<GenericError>> {
  let collection = database.collection::<Operator>("operators");

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
          "No se pudo eliminart el operador {}",
          oid
        ))))
      }
    }
    Err(error) => Err(Json(GenericError::new(&*format!("{:?}", error)))),
  }
}
