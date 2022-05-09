use rocket::futures::TryStreamExt;
use rocket::serde::json::Json;
use rocket::State;
use std::ops::Deref;

use mongodb::bson::{doc, oid::ObjectId};
use mongodb::{bson, Cursor};

use crate::database;
use crate::model::dosetrack::Dose;
use crate::utils::GenericError;

/*
1. Listar todas las dosis
2. listar las dosis por usuario
3. Agregar dosis al usuario
4. Que pasa si se carga mal una dosis o si se creo una que no iba?
*/

//Listar todas las dosis
//TODO: Falta hacer Join entre companies y Doses

#[get("/doses/<company_id>", format = "json")]
pub async fn get_by_company(
  company_id: String,
  database: &State<database::MongoDB>,
) -> Result<Json<Vec<Dose>>, Json<GenericError>> {
  let collection = database.collection::<Dose>("doses");
  let mut cursor: Cursor<Dose> = collection
    .find(
      doc! { "doses_by_company": ObjectId::parse_str(&company_id).unwrap() },
      None,
    )
    .await
    .unwrap();
  let mut doses: Vec<Dose> = Vec::new();

  while let Ok(Some(dose)) = cursor.try_next().await {
    doses.push(dose);
  }
  Ok(Json(doses))
}

//  Liste las dosis segun operador

#[get("/doses/operator/<operator_id>", format = "json")]
pub async fn get_by_operator(
  operator_id: String,
  database: &State<database::MongoDB>,
) -> Result<Json<Vec<Dose>>, Json<GenericError>> {
  let collection = database.collection::<Dose>("doses");
  let mut cursor: Cursor<Dose> = collection
    .find(
      doc! { "operator_id": ObjectId::parse_str(&operator_id).unwrap() },
      None,
    )
    .await
    .unwrap();
  let mut doses: Vec<Dose> = Vec::new();

  while let Ok(Some(dose)) = cursor.try_next().await {
    doses.push(dose);
  }
  Ok(Json(doses))
}

// TODO: Modificar el resto de las rutas segun este modelo para no usar clone en las estructuras
#[post("/dose", format = "json", data = "<new_dosis>")]
// ver qué significa data= new_dosis
pub async fn create(
  mut new_dosis: Json<Dose>,
  database: &State<database::MongoDB>,
) -> Result<Json<Dose>, GenericError> {
  let collection = database.collection::<Dose>("doses");

  if new_dosis._id.is_none() {
    new_dosis._id = Some(bson::oid::ObjectId::new());
    let _result = collection.insert_one(new_dosis.deref(), None).await;
  }

  Ok(Json(new_dosis.into_inner()))
}
