use rocket::{
    futures::TryStreamExt,
    futures::{FutureExt, StreamExt},
    http::Status,
    serde::json,
    serde::json::{json, Json},
    State,
};
use rocket_http::uri::fmt::UriDisplay;

use mongodb::{
    bson,
    bson::{doc, oid::ObjectId},
    options::{Collation, FindOptions},
    results::DeleteResult,
    Cursor,
};

use crate::database;
use crate::model::dosetrack::User;

#[get("/users/<company_id>")]
pub async fn get_all(company_id: String, database: &State<database::MongoDB>) -> (Status, String) {
    let collection = database.collection::<User>("users");
    let col = Collation::builder().locale("es").build();

    let filter = doc! {
     "company_id": ObjectId::parse_str(&company_id).unwrap(),
    };
    let options = FindOptions::builder()
        .collation(col)
        .sort(doc! {
          "name": 1i32
        })
        .build();

    let cursor: Cursor<User> = collection.find(filter, options).await.unwrap();
    let users = cursor.try_collect().await.unwrap_or_else(|_| vec![]);

    //let mut users: Vec<Result<User, _>> = cursor.collect().await;

    let json = json!(users).to_string();
    (Status::Ok, json)
}

#[get("/user/<id>", format = "json")]
pub async fn get(id: String, database: &State<database::MongoDB>) -> (Status, String) {
    let collection = database.collection::<User>("users");
    let user = collection
        .find_one(doc! { "_id": ObjectId::parse_str(&id).unwrap() }, None)
        .await
        .unwrap();

    match user {
        Some(u) => (Status::Ok, u.to_json()),
        None => (Status::NotFound, "{}".to_owned()),
    }
}

#[get("/user/email/<email>", format = "json")]
pub async fn get_by_email(email: String, database: &State<database::MongoDB>) -> (Status, String) {
    let collection = database.collection::<User>("users");
    let user = collection.find_one(doc! { "email": email }, None).await;

    match user {
        Ok(Some(u)) => (Status::Ok, u.to_json()),
        Ok(None) => (Status::NotFound, "{}".to_owned()),
        Err(e) => (Status::InternalServerError, e.to_string()),
    }
    // .await
    // .unwrap_or(None);

    // match user {
    //     Some(u) => (Status::Ok, json::to_string(&u).unwrap()),
    //     None => (Status::NotFound, "{}".to_owned()),
    // }
}

#[post("/user", format = "json", data = "<user>")]
pub async fn create_or_update(
    user: Json<User>,
    database: &State<database::MongoDB>,
) -> (Status, String) {
    let collection = database.collection::<User>("users");

    let new_user: User;

    if user._id.is_none() {
        new_user = User {
            _id: Some(bson::oid::ObjectId::new()),
            name: user.name.clone(),
            email: user.email.clone(),
            last_name: user.last_name.clone(),
            company_id: user.company_id.clone(),
            role: user.role.clone(),
            status: user.status.clone(),
        };
        let _result = collection.insert_one(&new_user, None).await;
    } else {
        new_user = User {
            _id: user._id,
            name: user.name.clone(),
            email: user.email.clone(),
            last_name: user.last_name.clone(),
            company_id: user.company_id.clone(),
            role: user.role.clone(),
            status: user.status.clone(),
        };

        let _result = collection
            .replace_one(doc! { "_id":  &user._id }, &new_user, None)
            .await;
    }
    //Ok(insert.inserted_id.to_string())

    (Status::Ok, new_user.to_json())
}

#[delete("/user/<id>")]
pub async fn delete(id: String, database: &State<database::MongoDB>) -> (Status, String) {
    let collection = database.collection::<User>("users");

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
                    Status::NotFound,
                    "No se pudo eliminart el usuario {}".to_owned(),
                )
            }
        }

        Err(error) => (Status::InternalServerError, error.to_string()),
    }
}
