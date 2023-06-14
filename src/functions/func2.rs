use serde_json::json;
use sqlx::{Postgres, Pool, PgPool};
use actix_web::{web::{self}, Responder, HttpResponse, http::StatusCode};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Customer{
    pub id: i32,
    pub name: String,
    pub address: String
}

#[doc = "docs"]
pub async fn get_customer(pool: web::Data<PgPool>) -> impl Responder {
//    sqlx::query_as!(Customer,
//         "SELECT id, name, address from customer")
//         .fetch_all( pool ).await.unwrap()

    let data = sqlx::query_as!(Customer,
    "SELECT id, name, address from customer")
    .fetch_all(pool.get_ref()).await.unwrap();

    let body = json!({
        "data": data
    });

    let mut response = HttpResponse::build(StatusCode::OK);
    response.content_type("application/json").json(body)

}
