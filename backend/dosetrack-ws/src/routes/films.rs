use rocket::serde::json::Json;
use rocket::State;
use std::ops::Deref;

use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::options::{Collation, FindOptions};
use mongodb::results::DeleteResult;
use mongodb::{bson, Cursor};

use crate::database;
use crate::model::dosetrack::Film;
use crate::utils::GenericError;
use rocket::futures::TryStreamExt;
use rocket::http::Status;
use rocket::response::status;

#[get("/by_company/<company_id>?<period>")]
pub async fn get_all(
    company_id: String,
    period: Option<String>,
    database: &State<database::MongoDB>,
) -> Result<Json<Vec<Document>>, GenericError> {
    let collection = database.collection::<Film>("films");

    let col = Collation::builder().locale("es").build();
    let mut query = vec![
        doc! {
            "$match": {
              "company_id": ObjectId::parse_str(&company_id).unwrap(),
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
            "$lookup": {
                "from": "periods",
                "localField": "period",
                "foreignField": "_id",
                "as": "period"
            }
        },
        doc! {
            "$unwind": {
                "path": "$operator",
                "preserveNullAndEmptyArrays": true
            }
        },
        doc! {
            "$unwind": {
                "path": "$period",
                "preserveNullAndEmptyArrays": true
            }
        },
    ];

    if period.is_some() {
        println!("{}", period.clone().unwrap_or("default".to_string()));

        query.push(doc! {
            "$match": {
                "period.period": period.unwrap(),
            }
        });
    }

    let mut films = Vec::new();
    let mut cursor: Cursor<Document> = collection.aggregate(query, None).await.unwrap();

    while let Ok(Some(film)) = cursor.try_next().await {
        films.push(film);
    }

    Ok(Json(films))
}

#[get("/<id>", format = "json")]
pub async fn get(
    id: String,
    database: &State<database::MongoDB>,
) -> Result<Json<Film>, GenericError> {
    let collection = database.collection::<Film>("films");
    let film = collection
        .find_one(doc! { "_id": ObjectId::parse_str(&id).unwrap() }, None)
        .await
        .unwrap();

    match film {
        Some(f) => Ok(Json(f)),
        None => Err(GenericError::new(
            Status::NotFound,
            &*format!("Film {} no encontrado", &id),
        )),
    }
}

// TODO: Modificar el resto de las rutas segun este modelo para no usar clone en las estructuras
#[post("/", format = "json", data = "<new_film>")]
pub async fn create_or_update(
    mut new_film: Json<Film>,
    database: &State<database::MongoDB>,
) -> status::Accepted<Json<Film>> {
    let collection = database.collection::<Film>("films");

    if new_film._id.is_none() {
        new_film._id = Some(bson::oid::ObjectId::new());
        let _result = collection.insert_one(new_film.deref(), None).await;
    } else {
        let _result = collection
            .replace_one(doc! { "_id":  &new_film._id }, new_film.deref(), None)
            .await;
    }

    status::Accepted(Some(Json(new_film.into_inner())))
    //Ok(Json(new_dosimeter.into_inner()))
}

#[delete("/<id>")]
pub async fn delete(
    id: String,
    database: &State<database::MongoDB>,
) -> Result<Json<u64>, GenericError> {
    let collection = database.collection::<Film>("films");

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
                    &*format!("No se pudo eliminar el film {}", oid),
                ))
            }
        }
        Err(error) => Err(GenericError::new(
            Status::NotFound,
            &*format!("{:?}", error),
        )),
    }
}
