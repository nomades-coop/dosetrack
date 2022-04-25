extern crate rocket;
use rocket::fs::FileServer;
use rocket::shield::Shield;
use rocket::*;

mod routes;

#[launch]
pub fn rocket_builder() -> _ {
  rocket::build()
    .attach(Shield::new())
    .mount("/", routes![routes::ping::ping])
    .mount(
      "/api",
      routes![
        routes::users::list_users,
        routes::users::create_user,
        routes::users::get_user,
        routes::users::update_user,
        routes::users::delete_user
      ],
    )
    .mount("/file", FileServer::from("static/"))
  //.mount("/", routes![echo_fn, fileserver]) // fileserver manually created
}
