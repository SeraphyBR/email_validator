#[macro_use] extern crate rocket;

#[cfg(test)]
mod tests;

mod controllers;
mod models;

use std::error::Error;

use rocket_contrib::json::Json;
use rocket_cors::CorsOptions;

use models::response::{Message, Response};
use controllers::health;
use controllers::mail;

pub struct PortConfig(u16);

#[get("/")]
fn index() -> Json<Response<String>> {
    Json(Response::ok())
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cors = CorsOptions::default().to_cors()?;

    let health = routes![health::index];
    let mail = routes![
        mail::validation_v1,
        mail::validation_v3
    ];

    rocket::build()
        .mount("/", routes![index])
        .mount("/health", health)
        .mount("/mail", mail)
        .manage(PortConfig(8080))
        .attach(cors)
        .launch().await?;

    Ok(())
}
