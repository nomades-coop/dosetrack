//#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use dosetrack_ws::rocket_builder;

#[launch]
fn ignite() -> _ {
    rocket_builder()
}
