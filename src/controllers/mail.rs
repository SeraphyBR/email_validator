use rocket_contrib::json::Json;
use crate::models::response::{Message, Response};
use crate::models::email::{EmailData, EmailQuery, Query};

#[post("/validation/v1", format = "json", data = "<query>")]
pub fn validation_v1(query: Json<Query>) -> Json<Response<EmailData>> {
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