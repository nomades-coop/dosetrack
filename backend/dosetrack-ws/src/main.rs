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

use rocket::fs::{relative, FileServer};
use rocket::shield::Shield;
use rocket_dyn_templates::Template;
use rocket_oauth2::OAuth2;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    rocket::build()
        .attach(Shield::new())
        .attach(database::init().await) // connect to the database
        .attach(OAuth2::<GoogleUserInfo>::fairing("google")) // setup OAuth
        .attach(Template::fairing()) // setup Tera templates
        .mount("/", FileServer::from(relative!("/static"))) // serving CSS
        .mount(
            "/",
            routes![
                blog_entries,
                users::get_all,
                users::get,
                users::create_or_update,
                users::delete,
                operators::get_all,
                operators::get,
                operators::create_or_update,
                operators::delete,
                doses::get_by_operator,
                doses::get_by_company,
                doses::create,
                //doses:detele?
            ],
        ) // public pages (blogposts)
        .mount(
            "/auth",
            routes![
                google_login,
                oauth_via_google,
                login_success,
                login_failure,
                logout
            ],
        ) // authentication
        .mount(
            "/admin",
            routes![
                admin_blog_entries,
                new_blog,
                new_blog_post,
                edit_blog,
                delete_blog
            ],
        ) // administration
        .launch()
        .await?;
    Ok(())
}
