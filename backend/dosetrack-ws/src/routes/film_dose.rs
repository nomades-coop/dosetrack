use rocket::serde::json::Json;
use rocket::State;
use rust_decimal::prelude::*;
use std::ops::Deref;

use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::options::{Collation, FindOptions};
use mongodb::results::DeleteResult;
use mongodb::{bson, Cursor};

use crate::database;
use crate::model::dosetrack::FilmDose;
use crate::utils::GenericError;
use rocket::futures::TryStreamExt;
use rocket::http::Status;
use rocket::response::status;

#[get("/<company_id>/<operator_id>", format = "json")]
pub async fn get_film_doses(
    company_id: String,
    operator_id: String,
    database: &State<database::MongoDB>,
) -> Result<Json<Vec<Document>>, GenericError> {
    let collection = database.collection::<FilmDose>("film_doses");

    let query = vec![
        doc! {
            "$match": {
              "company_id": ObjectId::parse_str(&company_id).unwrap(),
              "operator_id": ObjectId::parse_str(&operator_id).unwrap(),
            }
        },
        doc! {
            "$lookup": {
                "from": "operators",
                "localField": "operator_id",
                "foreignField": "_id",
                "as": "operator"
            }
        },
        doc! {
            "$set": {
                "operator": {
                    "$first": "$operator"
                }
            }
        },
    ];

    let mut doses = Vec::new();
    let mut cursor: Cursor<Document> = collection.aggregate(query, None).await.unwrap();

    while let Ok(Some(dose)) = cursor.try_next().await {
        doses.push(dose);
    }

    Ok(Json(doses))
}

#[post("/", format = "json", data = "<new_dosis>")]
// ver qué significa data= new_dosis
pub async fn create_or_update(
    mut new_dosis: Json<FilmDose>,
    database: &State<database::MongoDB>,
) -> Result<Json<FilmDose>, GenericError> {
    let collection = database.collection::<FilmDose>("film_doses");

    if new_dosis._id.is_none() {
        new_dosis._id = Some(bson::oid::ObjectId::new());
        _ = collection.insert_one(new_dosis.deref(), None).await;
    } else {
        let result = collection
            .replace_one(doc! { "_id":  &new_dosis._id }, new_dosis.deref(), None)
            .await
            .unwrap()
            .modified_count;

        if result == 0 {
            return Err(GenericError::new(
                Status::NotFound,
                &*format!("Film Dose {} no encontrado", new_dosis._id.unwrap()),
            ));
        }
    }

    Ok(Json(new_dosis.into_inner()))
}
