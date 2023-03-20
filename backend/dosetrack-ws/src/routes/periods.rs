use rocket::serde::json::{json, Json};
use rocket::State;
use serde::de::value::Error;
use std::ops::Deref;

use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::options::{Collation, FindOptions};
use mongodb::results::{DeleteResult, InsertOneResult};
use mongodb::{bson, Cursor};

use crate::database;
use crate::model::dosetrack::Period;
use crate::utils::GenericError;
use rocket::futures::{FutureExt, StreamExt, TryStreamExt};
use rocket::http::Status;
use rocket::response::status;

#[get("/by_company/<company_id>")]
pub async fn get_by_company(
    company_id: String,
    database: &State<database::MongoDB>,
) -> (Status, String) {
    let collection = database.collection::<Period>("periods");

    let col = Collation::builder().locale("es").build();

    let filter = doc! {
     "company_id": ObjectId::parse_str(&company_id).unwrap(),
    };
    let options = FindOptions::builder()
        .collation(col)
        .sort(doc! {
          "period": -1
        })
        .build();

    let mut cursor: Cursor<Period> = collection.find(filter, options).await.unwrap();

    let mut periods: Vec<Period> = Vec::new();
    while let Ok(Some(period)) = cursor.try_next().await {
        periods.push(period);
    }

    (Status::Ok, json!(periods).to_string())
}

#[get("/<id>", format = "json")]
pub async fn get(
    id: String,
    database: &State<database::MongoDB>,
) -> Result<Json<Period>, GenericError> {
    let collection = database.collection::<Period>("periods");
    let period = collection
        .find_one(doc! { "_id": ObjectId::parse_str(&id).unwrap() }, None)
        .await
        .unwrap();

    match period {
        Some(f) => Ok(Json(f)),
        None => Err(GenericError::new(
            Status::NotFound,
            &*format!("Período {} no encontrado", &id),
        )),
    }
}

#[post("/", format = "json", data = "<new_period>")]
pub async fn create_or_update(
    mut new_period: Json<Period>,
    database: &State<database::MongoDB>,
) -> (Status, String) {
    let collection = database.collection::<Period>("periods");
    // let result: Result<InsertOneResult, mongodb::error::Error>;
    let result: InsertOneResult;

    if new_period.id.is_none() {
        let filter = doc! {"period": &new_period.period};
        let period = collection.find_one(filter, None).await.unwrap_or(None);

        if !period.is_none() {
            return (
                Status::InternalServerError,
                json!(doc! {"error": 902, "msg": "El período ya existe"}).to_string(),
            );
        }

        new_period.id = Some(bson::oid::ObjectId::new());

        match collection.insert_one(new_period.deref(), None).await {
            Ok(p) => result = p,
            Err(e) => {
                return (
                    Status::InternalServerError,
                    json!(doc! {"error": 901, "msg": e.to_string()}).to_string(),
                );
            }
        };
        println!("Nuevo Periodo: {}", result.inserted_id);
        // new_period.id = result.unwrap().inserted_id.as_object_id();
        new_period.id = result.inserted_id.as_object_id();
    } else {
        println!("Periodo a actualizar: {}", &new_period.id.unwrap());
        let _result = collection
            .replace_one(
                doc! { "_id": &new_period.id.unwrap() },
                new_period.deref(),
                None,
            )
            .await
            .unwrap();

        if _result.modified_count == 0 {
            return (
                Status::NotFound,
                json!(doc! {"error": 903, "msg": "No hay nada que actualizar".to_string()})
                    .to_string(),
            );
        }

        println!("Periodos modificados: {}", _result.modified_count);
    }

    (Status::Accepted, json!(new_period.into_inner()).to_string())

    // TODO: Validar que no se repita periodo para la misma empresa
}

#[delete("/<id>")]
pub async fn delete(
    id: String,
    database: &State<database::MongoDB>,
) -> Result<Json<u64>, GenericError> {
    let collection = database.collection::<Period>("periods");

    let result: Result<ObjectId, mongodb::bson::oid::Error> = ObjectId::parse_str(&id);
    let delete_result: Result<DeleteResult, mongodb::error::Error>;

    match result {
        Ok(oid) => {
            delete_result = collection.delete_one(doc! { "_id": oid }, None).await;
            let records_deleted = delete_result.unwrap().deleted_count;
            if records_deleted > 0 {
                Ok(Json(records_deleted))
            } else {
                Err(GenericError::new(
                    Status::NotFound,
                    &*format!("No se pudo eliminar el periodo {}", oid),
                ))
            }
        }
        Err(error) => Err(GenericError::new(
            Status::NotFound,
            &*format!("{:?}", error),
        )),
    }
}
