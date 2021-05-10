#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

mod controllers;
mod models;

use rocket::State;
use rocket::fairing::AdHoc;
use rocket_contrib::json::Json;
use rocket_cors::CorsOptions;

use models::response::{Message, Response};
use controllers::health;

pub struct PortConfig(u16);

fn build_rocket() -> rocket::Rocket {
    let cors = CorsOptions::default().to_cors().unwrap();

    let health = routes![health::index];

    rocket::ignite()
        .mount("/", routes![index])
        .mount("/health", health)
        .attach(AdHoc::on_attach("Port Config", |rocket| {
            let port = rocket.config().port;
            Ok(rocket.manage(PortConfig(port)))
        }))
        .attach(cors)
}

#[get("/")]
fn index() -> Json<Response<String>> {
    Json(Response::ok())
}


fn main() {
    let rocket = build_rocket();
    rocket.launch();
}
