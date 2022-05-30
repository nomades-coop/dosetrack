#![allow(dead_code)]
#![allow(unused_variables, unused_imports)]

#[macro_use]
extern crate rocket;

mod auth;
mod database;
mod model;
mod routes;
mod utils;

use auth::GoogleUserInfo;
use routes::*;
use std::error::Error;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::{relative, FileServer};
use rocket::http::Header;
use rocket::http::{ContentType, Method, Status};
use rocket::shield::Shield;
use rocket::{Request, Response};
use rocket_dyn_templates::Template;
use rocket_oauth2::OAuth2;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, DELETE, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_status(Status::Ok);
    }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _rocket = rocket::build()
        .attach(Shield::new())
        .attach(database::init().await) // connect to the database
        .attach(CORS)
        .mount("/", FileServer::from(relative!("/static"))) // serving CSS
        .mount(
            "/dose",
            routes![doses::get_doses, doses::get_by_operator, doses::create,],
        )
        .mount(
            "/film",
            routes![
                films::create_or_update,
                films::delete,
                films::get,
                films::get_all,
            ],
        )
        .mount(
            "/",
            routes![
                users::get_all,
                users::get,
                users::create_or_update,
                users::delete,
                operators::get_by_company,
                operators::get,
                operators::create_or_update,
                operators::delete,
                companies::get_all,
                companies::get,
                companies::create_or_update,
                companies::delete,
                dosimeters::get_all,
                dosimeters::get,
                dosimeters::create_or_update,
                dosimeters::delete
            ],
        )
        .launch()
        .await?;
    Ok(())
}
