use rocket::serde::json::Json;
use rocket::State;

use mongodb::bson::{doc, oid::ObjectId};
use mongodb::results::DeleteResult;
use mongodb::{bson, Cursor};

use crate::database;
use crate::model::dosetrack::{Dosimeter, User, UserRole, UserStatus};
use crate::utils::GenericError;

//  Liste las dosis segun operador

// #[get("/doses/<operator_id>", format = "json")]
// pub async fn get(
//   operator_id: String,
//   database: &State<database::MongoDB>,
// ) -> Result<Json<User>, Json<GenericError>> {

// }

//  Crear un ruta/endpoint que cree dosis en la db

// #[post("/dose", format = "json", data = "<dose>")]
// pub async fn create_or_update(
//   dose: Json<Dose>,
//   database: &State<database::MongoDB>,
// ) -> Result<Json<Dose>, GenericError> {

// }
