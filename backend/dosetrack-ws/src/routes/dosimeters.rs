use rocket::serde::json::Json;
use rocket::State;
use std::ops::Deref;

use mongodb::bson::{doc, oid::ObjectId};
use mongodb::results::DeleteResult;
use mongodb::{bson, Cursor};

use crate::database;
use crate::model::dosetrack::Dosimeter;
use crate::utils::GenericError;
use rocket::futures::TryStreamExt;
use rocket::http::Status;
use rocket::response::status;

#[get("/dosimeters")]
pub async fn get_all(
    database: &State<database::MongoDB>,
) -> Result<Json<Vec<Dosimeter>>, GenericError> {
    let collection = database.collection::<Dosimeter>("dosimeters");
    let mut cursor: Cursor<Dosimeter> = collection.find(None, None).await.unwrap();

    let mut dosimeters: Vec<Dosimeter> = Vec::new();
    while let Ok(Some(dosimeter)) = cursor.try_next().await {
        dosimeters.push(dosimeter);
    }

    Ok(Json(dosimeters))
}

#[get("/dosimeter/<id>", format = "json")]
pub async fn get(
    id: String,
    database: &State<database::MongoDB>,
) -> Result<Json<Dosimeter>, GenericError> {
    let collection = database.collection::<Dosimeter>("dosimeters");
    let dosimeter = collection
        .find_one(doc! { "_id": ObjectId::parse_str(&id).unwrap() }, None)
        .await
        .unwrap();

    match dosimeter {
        Some(o) => Ok(Json(o)),
        None => Err(GenericError::new(
            Status::NotFound,
            &*format!("Dosimetro {} no encontrado", &id),
        )),
    }
}

// TODO: Modificar el resto de las rutas segun este modelo para no usar clone en las estructuras
#[post("/dosimeter", format = "json", data = "<new_dosimeter>")]
pub async fn create_or_update(
    mut new_dosimeter: Json<Dosimeter>,
    database: &State<database::MongoDB>,
) -> status::Accepted<Json<Dosimeter>> {
    let collection = database.collection::<Dosimeter>("dosimeters");

    if new_dosimeter._id.is_none() {
        new_dosimeter._id = Some(bson::oid::ObjectId::new());
        let _result = collection.insert_one(new_dosimeter.deref(), None).await;
    } else {
        let _result = collection
            .replace_one(
                doc! { "_id":  &new_dosimeter._id },
                new_dosimeter.deref(),
                None,
            )
            .await;
    }

    status::Accepted(Some(Json(new_dosimeter.into_inner())))
    //Ok(Json(new_dosimeter.into_inner()))
}

#[delete("/dosimeter/<id>")]
pub async fn delete(
    id: String,
    database: &State<database::MongoDB>,
) -> Result<Json<u64>, GenericError> {
    let collection = database.collection::<Dosimeter>("dosimeters");

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
                    Status::InternalServerError,
                    &*format!("No se pudo eliminart el dosimetro {}", oid),
                ))
            }
        }
        Err(error) => Err(GenericError::new(
            Status::InternalServerError,
            &*format!("{:?}", error),
        )),
    }
}
