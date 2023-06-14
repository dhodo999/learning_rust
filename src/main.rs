use std::{collections::HashMap};
use actix_web::{HttpServer, App, web, HttpResponse, Responder, post};
use sqlx::{postgres::PgPoolOptions, Result};
use tutorial::{get_location, get_customer};
use dotenv::dotenv;


// #[derive(Debug)]
// struct Murid{
//     nama: String,
//     umur: i32,
//     nilai: f64
// }

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:dashdho2005@localhost/tutor").await
        .expect("connection failed!");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/balls", web::get().to(get_customer))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// fn hello(nama: String) -> String {
//     format!("hello, {}", nama)
// }

// fn test() {
//     let nama = "Dhodo";
//     let jurusan = "RPL".to_string();

//     // let clone = jurusan.clone();

//     // let umur = 17;
//     let pria = false;
//     // let nilai = 8.5;

//     // let array = vec!["a", "b"];
//     let mut object: HashMap<String, i32> = HashMap::new();
//     object.insert("umur".to_string(), 20);

//     let murid = Murid{
//         nama: "Dhodo".to_string(), 
//         umur: 17,
//         nilai: 8.5
//     };

//     // println!("Hello, {kelas} {clone}!");
//     // println!("{:?}", murid);
//     // println!("{:#?}", array);

//     let hello = hello(nama.to_string());

//     if pria {
//         println!("{}", hello);
//     } else {
//         println!("halo mba");
//     }

//     for a in 0..10 {
//         println!("{a}");
//     }

//     println!("umur: {}", murid.umur);

//     println!("func: {}", get_location());
// }
