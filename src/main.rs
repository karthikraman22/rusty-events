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

pub mod config;
pub mod gateway;
pub mod transport;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let cfg = config::Config::load();
    let gw = gateway::bootstrap(&cfg);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/hello", web::get().to(hello))
    })
    .bind(cfg.bind_address())?
    .run()
    .await
}