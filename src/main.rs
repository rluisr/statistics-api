#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

mod models;
mod uuid;
mod db;

use rocket::{State};
use rocket::response::Redirect;
use rocket_contrib::json::Json;

use crate::uuid::{UUID};
use crate::models::Response;

#[get("/")]
fn index() -> Redirect {
    Redirect::to("https://github.com/rluisr/stats-api-rust")
}

#[get("/health_check")]
fn health_check() -> &'static str {
    ":)"
}

#[post("/uuid", data = "<register>")]
fn register(register: Json<UUID>, connection: State<mysql::Pool>) -> Json<Response> {
    Json(UUID::register(register, connection))
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/", routes![index, health_check, register])
        .launch();
}
