use serde::{Serialize, Deserialize};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseData {
    name:  String,
    data:  String
}

extern crate base64;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

async fn hello() -> impl Responder {
    let data: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(1024)
        .collect();

    let response = serde_json::to_string(&ResponseData{ name: String::from("Hello"), data: data}).unwrap();
    HttpResponse::Ok().body(response)
}

use gateway;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let cfg = gateway::Config{listen_address: "", port:8088};

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/hello", web::get().to(hello))
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}