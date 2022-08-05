use chrono::prelude::*;
use rocket::data::ToByteUnit;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::Data;
use rocket::State;
use std::ops::Deref;

use mongodb::{
    bson,
    bson::{doc, oid::ObjectId, DateTime},
    options::{Collation, FindOptions},
    results::DeleteResult,
    Cursor,
};

use crate::database;
use crate::model::dosetrack::{Company, CompanyStatus, Registration, User, UserRole, UserStatus};
use crate::utils::GenericError;
use rocket::futures::TryStreamExt;
use rocket::http::Status;

async fn company_by_cuit(cuit: &String, database: &State<database::MongoDB>) -> Option<Company> {
    let collection = database.collection::<Company>("companies");
    let company = collection
        .find_one(doc! { "cuit": cuit }, None)
        .await
        .unwrap();

    company
}

async fn user_by_email(email: &String, database: &State<database::MongoDB>) -> Option<User> {
    let collection = database.collection::<User>("users");
    let user = collection
        .find_one(doc! { "email": email }, None)
        .await
        .unwrap();

    user
}

#[post("/registration", format = "json", data = "<new_registration>")]
pub async fn new(
    new_registration: Json<Registration>,
    database: &State<database::MongoDB>,
) -> (Status, String) {
    let users = database.collection::<User>("users");
    let companies = database.collection::<Company>("companies");

    let company = Company {
        _id: Some(bson::oid::ObjectId::new()),
        name: new_registration.company_name.clone(),
        cuit: new_registration.company_cuit.clone(),
        operators: None,
        status: CompanyStatus::Enabled,
    };

    if company_by_cuit(&company.cuit, database).await.is_some() {
        return (Status::Conflict, "Company already registered".to_string());
    }

    if user_by_email(&new_registration.user_email.clone().unwrap(), database)
        .await
        .is_some()
    {
        return (
            Status::Conflict,
            "User already registered by another company".to_string(),
        );
    }

    let result = companies.insert_one(company, None).await;

    if result.is_err() {
        return (
            Status::InternalServerError,
            "Error creating company".to_string(),
        );
    }

    let new_user = User {
        _id: Some(bson::oid::ObjectId::new()),
        name: new_registration.user_name.clone(),
        email: new_registration.user_email.clone(),
        last_name: new_registration.user_last_name.clone(),
        company_id: result.unwrap().inserted_id.as_object_id().unwrap(),
        role: UserRole::Admin,
        status: UserStatus::Enabled,
    };

    let result = users.insert_one(new_user, None).await;

    match result {
        Ok(ior) => (Status::Ok, new_registration.to_json()),
        Err(e) => (Status::UnprocessableEntity, e.to_string()),
    }
}
