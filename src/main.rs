#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

mod models;
mod uuid;
mod db;

use rocket::{State};
use rocket_contrib::json::Json;

use crate::uuid::{UUID};
use crate::models::Register;
use crate::models::Response;

#[get("/")]
fn index() -> &'static str {
    ":)"
}

#[post("/uuid", data = "<register>")]
fn register(register: Json<Register>, connection: State<mysql::Pool>) -> Json<Response> {
    Json(UUID::register(register, connection))
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/", routes![index, register])
        .launch();
}
