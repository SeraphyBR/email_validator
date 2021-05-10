use serde::{Serialize, Deserialize};

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
    pub deriverable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub catch_all: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gibberish: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam: Option<bool>
}

impl EmailData {}

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