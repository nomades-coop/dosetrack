use rocket::futures::TryStreamExt; //lo usa try_next en el cursor
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use std::ops::Deref;

use mongodb::{
    bson,
    bson::{doc, oid::ObjectId, DateTime, Document},
    options::FindOptions,
    Cursor,
};

use crate::database;
use crate::model::dosetrack::Dose;
use crate::utils::GenericError;

/*
1. Listar todas las dosis
2. listar las dosis por usuario
3. Agregar dosis al usuario
4. Que pasa si se carga mal una dosis o si se creo una que no iba?
*/

//Listar  las dosis
//TODO: Falta hacer Join entre companies y Doses

#[get("/<company_id>/<operator_id>", format = "json")]
pub async fn get_doses(
    company_id: String,
    operator_id: String,
    database: &State<database::MongoDB>,
) -> Result<Json<Vec<Document>>, GenericError> {
    let collection = database.collection::<Dose>("doses");

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
            "$lookup": {
                "from": "dosimeters",
                "localField": "dosimeter_id",
                "foreignField": "_id",
                "as": "dosimeter"
            }
        },
        doc! {
            "$set": {
                "operator": {
                    "$first": "$operator"
                },
                "dosimeter": {
                    "$first": "$dosimeter"
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

    // let filter = doc! {
    //  "company_id": ObjectId::parse_str(&company_id).unwrap(),
    //  "operator_id": ObjectId::parse_str(&operator_id).unwrap(),
    // };
    // let find_options = FindOptions::builder()
    //   .projection(doc! {
    //     "_id": 1,
    //     "company_id": 1,
    //     "operator_id": 1,
    //     "dose": 1,
    //     "when": 1
    //   })
    //   .sort(doc! {
    //     "when": 1
    //   })
    //   .build();

    // let mut cursor: Cursor<Dose> = collection.find(filter, find_options).await.unwrap();
    // let mut doses: Vec<Dose> = Vec::new();

    // while let Ok(Some(dose)) = cursor.try_next().await {
    //   doses.push(dose);
    // }
    // Ok(Json(doses))
}

//  Liste las dosis segun operador
#[get("/operator/<operator_id>", format = "json")]
pub async fn get_by_operator(
    operator_id: String,
    database: &State<database::MongoDB>,
) -> Result<Json<Vec<Dose>>, GenericError> {
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
#[post("/", format = "json", data = "<new_dosis>")]
// ver qué significa data= new_dosis
pub async fn create(
    mut new_dosis: Json<Dose>,
    database: &State<database::MongoDB>,
) -> Result<Json<Dose>, GenericError> {
    let collection = database.collection::<Dose>("doses");

    if new_dosis._id.is_none() {
        new_dosis._id = Some(bson::oid::ObjectId::new());
        new_dosis.when = DateTime::now();
        let _result = collection.insert_one(new_dosis.deref(), None).await;
    }

    Ok(Json(new_dosis.into_inner()))
}
