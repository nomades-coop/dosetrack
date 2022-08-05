use rocket::serde::json::Json;
use rocket::State;
use std::ops::Deref;

use mongodb::bson::{doc, oid::ObjectId};
use mongodb::results::DeleteResult;
use mongodb::{bson, Cursor};

use crate::database;
use crate::model::dosetrack::Company;
use crate::utils::GenericError;
use rocket::futures::TryStreamExt;
use rocket::http::Status;

#[get("/companies")]
pub async fn get_all(
    database: &State<database::MongoDB>,
) -> Result<Json<Vec<Company>>, GenericError> {
    let collection = database.collection::<Company>("companies");
    let mut cursor: Cursor<Company> = collection.find(None, None).await.unwrap();

    let mut companies: Vec<Company> = Vec::new();
    while let Ok(Some(company)) = cursor.try_next().await {
        companies.push(company);
    }

    Ok(Json(companies))
}

#[get("/company/<id>", format = "json")]
pub async fn get(
    id: String,
    database: &State<database::MongoDB>,
) -> Result<Json<Company>, GenericError> {
    let collection = database.collection::<Company>("companies");
    let company = collection
        .find_one(doc! { "_id": ObjectId::parse_str(&id).unwrap() }, None)
        .await
        .unwrap();

    match company {
        Some(o) => Ok(Json(o)),
        None => Err(GenericError::new(
            Status::NotFound,
            &*format!("Empresa {} no encontrada", &id),
        )),
    }
}

// TODO: Modificar el resto de las rutas segun este modelo para no usar clone en las estructuras
#[post("/company", format = "json", data = "<new_company>")]
pub async fn create_or_update(
    mut new_company: Json<Company>,
    database: &State<database::MongoDB>,
) -> Result<Json<Company>, GenericError> {
    let collection = database.collection::<Company>("companies");

    if new_company._id.is_none() {
        new_company._id = Some(bson::oid::ObjectId::new());
        let _result = collection.insert_one(new_company.deref(), None).await;
    } else {
        let _result = collection
            .replace_one(doc! { "_id":  &new_company._id }, new_company.deref(), None)
            .await;
    }

    Ok(Json(new_company.into_inner()))
}

#[delete("/company/<id>")]
pub async fn delete(
    id: String,
    database: &State<database::MongoDB>,
) -> Result<Json<u64>, GenericError> {
    let collection = database.collection::<Company>("companies");

    let result: Result<ObjectId, mongodb::bson::oid::Error> = ObjectId::parse_str(&id);
    let delete_result: Result<DeleteResult, mongodb::error::Error>;

    match result {
        Ok(oid) => {
            delete_result = collection.delete_one(doc! { "_id": oid }, None).await;
            let users_deleted = delete_result.unwrap().deleted_count;
            if users_deleted > 0 {
                Ok(Json(users_deleted))
            } else {
                Err(GenericError::new(
                    Status::InternalServerError,
                    &*format!("No se pudo eliminart la mepresa {}", oid),
                ))
            }
        }
        Err(error) => Err(GenericError::new(
            Status::InternalServerError,
            &*format!("{:?}", error),
        )),
    }
}
