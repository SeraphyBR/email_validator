use rocket_contrib::json::Json;
use rocket::State;
use sqlx::PgPool;
use crate::models::response::{Response, EvaResponse};
use crate::models::email::{EmailData, EmailQueryDB, Query, EmailDataV1, EmailDataV3};
use rocket::futures::{stream,StreamExt};
use reqwest;

#[post("/validation/v1", format = "json", data = "<query>")]
pub async fn validation_v1(query: Json<Query>, db: State<'_, PgPool>) -> Json<Response<EmailData>> {
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
        sqlx::query!("
            INSERT INTO email_data_v1(email_address, domain, valid_syntax)
            VALUES($1,$2,$3)
            ",
            data.email_address,
            data.domain,
            data.valid_syntax
        ).execute(db.inner()).await.unwrap();
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
pub async fn validation_v3(query: Json<Query>, db: State<'_, PgPool>) -> Json<Response<EmailData>> {
    let client = reqwest::Client::new();
    let mut response = Response::ok();
    match query.into_inner() {
        Query::One(eq) => {
            response.with_result(get_eva_response(&client, eq.email_address, &db).await)
        },
        Query::More(emails) => {
            // https://stackoverflow.com/questions/51044467/how-can-i-perform-parallel-asynchronous-http-get-requests-with-reqwest
            // Aqui eu itero por cada email vindo da requisição, retornando para cada, um future
            // Depois em buffer_unordered todas as futures serão executadas de forma concorrente, com o limite de 24 futures concorrentes
            let results = stream::iter(emails).map(|eq| {
                get_eva_response(&client, eq.email_address, &db)
            }).buffer_unordered(24);

            let mut results = results.collect::<Vec<EmailData>>().await;
            response.with_results(&mut results);
        }
    };

    Json(response)
}

async fn get_eva_response(client: &reqwest::Client, email: String, db: &PgPool) -> EmailData {
    let url = format!("https://api.eva.pingutil.com/email?email={}", email);
    let resp = client.get(&url).send().await.unwrap()
        .json::<EvaResponse>()
        .await.unwrap();
    let data = resp.data;
    sqlx::query!("
        INSERT INTO email_data_v3(
            email_address, domain, valid_syntax, disposable, webmail,
            deliverable, catch_all, gibberish, spam
        )
        VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9)
        ",
        data.email_address,
        data.domain,
        data.valid_syntax,
        data.disposable,
        data.webmail,
        data.deliverable,
        data.catch_all,
        data.gibberish,
        data.spam
    ).execute(db).await.unwrap();
    data
}

#[get("/db/v1?<query..>")]
pub async fn database_v1(query: EmailQueryDB, db: State<'_, PgPool>) -> Json<Response<EmailDataV1>> {

    // https://stackoverflow.com/questions/45938494/postgresql-conditional-where-clause
    let mut emails = sqlx::query_as!(EmailDataV1,
        "SELECT *
        FROM email_data_v1
        WHERE
            ($1::int IS NULL OR id = $1::int) AND
            ($2::varchar IS NULL OR email_address = $2::varchar) AND
            ($3::varchar IS NULL OR domain = $3::varchar) AND
            ($4::bool IS NULL OR valid_syntax = $4::bool)
        ", query.id, query.email_address, query.domain, query.valid_syntax)
        .fetch_all(db.inner())
        .await
        .unwrap();

    let mut response = Response::ok();
    response.with_results(&mut emails);
    Json(response)
}

#[get("/db/v3?<query..>")]
pub async fn database_v3(query: EmailQueryDB, db: State<'_, PgPool>) -> Json<Response<EmailDataV3>> {
    let mut emails = sqlx::query_as!(EmailDataV3,
        "SELECT *
        FROM email_data_v3
        WHERE
            ($1::int IS NULL OR id = $1::int) AND
            ($2::varchar IS NULL OR email_address = $2::varchar) AND
            ($3::varchar IS NULL OR domain LIKE $3::varchar) AND
            ($4::bool IS NULL OR valid_syntax = $4::bool) AND
            ($5::bool IS NULL OR valid_syntax = $5::bool) AND
            ($6::bool IS NULL OR valid_syntax = $6::bool) AND
            ($7::bool IS NULL OR valid_syntax = $7::bool) AND
            ($8::bool IS NULL OR valid_syntax = $8::bool) AND
            ($9::bool IS NULL OR valid_syntax = $9::bool) AND
            ($10::bool IS NULL OR valid_syntax = $10::bool)
        ",
        query.id,
        query.email_address,
        query.domain,
        query.valid_syntax,
        query.disposable,
        query.webmail,
        query.deliverable,
        query.catch_all,
        query.gibberish,
        query.spam)
        .fetch_all(db.inner())
        .await
        .unwrap();

    let mut response = Response::ok();
    response.with_results(&mut emails);
    Json(response)
}