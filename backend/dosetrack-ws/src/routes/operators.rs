use chrono::prelude::*;
use rocket::futures::StreamExt;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::State;
use std::ops::Deref;

use mongodb::{
    bson,
    bson::{doc, oid::ObjectId, DateTime, Document},
    options::{Collation, FindOptions},
    results::DeleteResult,
    Cursor,
};

use crate::database;
use crate::model::dosetrack::Dose;
use crate::model::dosetrack::Operator;
use crate::model::dosetrack::Period;
use crate::utils::GenericError;
use rocket::futures::TryStreamExt;
use rocket::http::Status;

#[get("/operators/doses/<company_id>/<period>")]
pub async fn get_by_company_with_doses(
    company_id: String,
    period: String,
    database: &State<database::MongoDB>,
) -> (Status, String) {
    let pipeline = vec![
        doc! {
            "$match": doc! {
                "company_id": ObjectId::parse_str(&company_id).unwrap()
            }
        },
        doc! {
            "$lookup": doc! {
                "from": "operators",
                "localField": "company_id",
                "foreignField": "company_id",
                "let": doc! {
                    "pid": "$_id"
                },
                "as": "operators",
                "pipeline": [
                    doc! {
                        "$lookup": doc! {
                            "from": "film_doses",
                            "let": doc! {
                                "pid": "$$pid"
                            },
                            "localField": "_id",
                            "foreignField": "operator_id",
                            "as": "film_doses",
                            "pipeline": [
                                doc! {
                                    "$match": doc! {
                                        "$expr": doc! {
                                            "$eq": [
                                                "$period_id",
                                                "$$pid"
                                            ]
                                        }
                                    }
                                }
                            ]
                        }
                    }
                ]
            }
        },
    ];

    let collection = database.collection::<Period>("periods");
    let mut cursor = collection.aggregate(pipeline, None).await.unwrap();
    // get total items in cursor
    // let total = cursor.count().await;
    // println!("Comientza el while {}", total);

    let mut documents: Vec<Document> = Vec::new();

    // fill operators vector with cursor data
    while let Ok(Some(result)) = cursor.try_next().await {
        documents.push(bson::from_document(result).unwrap());
    }

    (Status::Ok, json!(documents).to_string())
}

#[get("/operators/<company_id>")]
pub async fn get_by_company(
    company_id: String,
    database: &State<database::MongoDB>,
) -> (Status, String) {
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

    (Status::Ok, json!(operators).to_string())
}

#[get("/operators/overdose/<company_id>")]
pub async fn overdose(company_id: String, database: &State<database::MongoDB>) -> (Status, String) {
    let pipeline = vec![
        doc! {
            "$match": {
                "company_id": ObjectId::parse_str(&company_id).unwrap(),
                "when": {"$gte": "ISODate('2022-05-01T00:00:00.000Z')"}
            }
        },
        doc! {
            "$group": {
                "_id": "$operator_id",
                "totalDosis": {
                    "$sum": "$dose"
                }
            }
        },
        doc! {
            "$lookup": {
                "from": "operators",
                "localField": "_id",
                "foreignField": "_id",
                "as": "operator"
              }
        },
        doc! {
            "$unwind": {
                "path": "$operator"
            }
        },
        doc! {
            "$sort": {
                "totalDose": -1
            }
        },
    ];

    let collection = database.collection::<Dose>("doses");
    let mut cursor = collection.aggregate(pipeline, None).await.unwrap();
    // get total items in cursor
    // let total = cursor.count().await;
    // println!("Comientza el while {}", total);

    let mut documents: Vec<Document> = Vec::new();

    // fill operators vector with cursor data
    while let Ok(Some(result)) = cursor.try_next().await {
        documents.push(bson::from_document(result).unwrap());
    }

    (Status::Ok, json!(documents).to_string())
}

#[get("/operator/<id>", format = "json")]
pub async fn get(id: String, database: &State<database::MongoDB>) -> (Status, String) {
    let collection = database.collection::<Operator>("operators");
    let operator = collection
        .find_one(doc! { "_id": ObjectId::parse_str(&id).unwrap() }, None)
        .await
        .unwrap();

    match operator {
        Some(o) => (Status::Ok, json!(o).to_string()),
        None => (Status::NotFound, format!("Operador {} no encontrado", &id)),
    }
}

// TODO: Modificar el resto de las rutas segun este modelo para no usar clone en las estructuras
#[post("/operator", format = "json", data = "<new_operator>")]
pub async fn create_or_update(
    mut new_operator: Json<Operator>,
    database: &State<database::MongoDB>,
) -> (Status, String) {
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

    (Status::Ok, json!(new_operator.into_inner()).to_string())
}

#[delete("/operator/<id>")]
pub async fn delete(id: String, database: &State<database::MongoDB>) -> (Status, String) {
    let collection = database.collection::<Operator>("operators");

    let result: Result<ObjectId, mongodb::bson::oid::Error> = ObjectId::parse_str(&id);
    let delete_result: Result<DeleteResult, mongodb::error::Error>;

    match result {
        Ok(oid) => {
            delete_result = collection.delete_one(doc! { "_id": oid }, None).await;
            let users_deleted = delete_result.unwrap().deleted_count;
            if users_deleted > 0 {
                (Status::Ok, json!(users_deleted).to_string())
            } else {
                (
                    Status::InternalServerError,
                    format!("No se pudo eliminart el operador {}", oid),
                )
            }
        }
        Err(error) => (Status::InternalServerError, format!("{:?}", error)),
    }
}
