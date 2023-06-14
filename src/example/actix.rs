use std::sync::Mutex;

use actix_web::{HttpServer, App, web, HttpResponse, Responder, post};
use initial_project::Customer;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;

struct PoolState {
    pool: Mutex<Pool<Postgres>>, // <- Mutex is necessary to mutate safely across threads
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello(data: web::Data<PoolState>) -> impl Responder {
    let pool = data.pool.lock().unwrap().clone();
    let customer = sqlx::query_as!(Customer,
        "SELECT id, name, address from customer")
        .fetch_all( &pool ).await.unwrap();

    HttpResponse::Ok().json(customer)
    // HttpResponse::Ok().body("tes balikan")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:dashdho2005@localhost/tutor").await
        .expect("connection failed!");
    

    HttpServer::new(move || {
        let pool_state = web::Data::new(PoolState {
            pool: Mutex::new(pool.clone()),
        });

        App::new()
        .app_data(pool_state)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
