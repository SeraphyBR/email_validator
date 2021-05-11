use rocket_contrib::json::Json;
use sqlx::PgPool;
use rocket::State;
use crate::models::response::{Message, Response, EvaResponse};
use crate::models::email::{EmailData, EmailQuery, Query};
use std::collections::HashMap;
use std::error::Error;
use reqwest;

#[post("/validation/v1", format = "json", data = "<query>")]
pub async fn validation_v1(query: Json<Query>) -> Json<Response<EmailData>> {
    let mut response = Response::ok();
    let emails = match query.into_inner() {
        Query::One(eq) => vec![eq],
        Query::More(vec) => vec
    };
    for eq in emails {
        let mut data = EmailData::default();
        data.email_address = eq.email_address.clone();
        data.domain = "mail".to_string();
        data.valid_syntax = verify_syntax(&eq.email_address);
        response.with_result(data);
    }
    Json(response)
}

fn verify_syntax(email: &String) -> bool {
    email.ends_with(".com.br") ||
    email.ends_with(".com") ||
    email.ends_with(".gov.br") ||
    email.ends_with(".org")
}


#[post("/validation/v3", format = "json", data = "<query>")]
pub async fn validation_v3(query: Json<Query>) -> Json<Response<EmailData>> {
    let mut response = Response::ok();
    let emails = match query.into_inner() {
        Query::One(eq) => vec![eq],
        Query::More(vec) => vec
    };
    for eq in emails {
         let resp = get_response_from_eva(eq.email_address).await.unwrap();
        response.with_result(resp.data);
    }
    Json(response)
}

async fn get_response_from_eva(email: String) -> Result<EvaResponse, Box<dyn Error>> {
    let url = format!("https://api.eva.pingutil.com/email?email={}", email);
    let resp = reqwest::get(&url).await?
        .json::<EvaResponse>()
        .await?;
    Ok(resp)
}