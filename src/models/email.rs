use serde::{Serialize, Deserialize};
use rocket::form::Form;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct EmailData {
    pub email_address: String,
    pub domain: String,
    pub valid_syntax: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disposable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub webmail: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliverable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub catch_all: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gibberish: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam: Option<bool>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmailQuery {
    pub email_address: String,
    pub domain: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Query {
    One(EmailQuery),
    More(Vec<EmailQuery>)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmailDataV1 {
    pub id: i32,
    pub email_address: String,
    pub domain: String,
    pub valid_syntax: bool,
    pub created_at: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmailDataV3 {
    pub id: i32,
    pub email_address: String,
    pub domain: String,
    pub valid_syntax: bool,
    pub disposable: bool,
    pub webmail: bool,
    pub deliverable: bool,
    pub catch_all: bool,
    pub gibberish: bool,
    pub spam: bool,
    pub created_at: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Clone, Default, FromForm)]
pub struct EmailQueryDB {
    pub id: Option<i32>,
    pub email_address: Option<String>,
    pub domain: Option<String>,
    pub valid_syntax: Option<bool>,
    pub disposable: Option<bool>,
    pub webmail: Option<bool>,
    pub deliverable: Option<bool>,
    pub catch_all: Option<bool>,
    pub gibberish: Option<bool>,
    pub spam: Option<bool>
}