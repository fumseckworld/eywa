#![allow(clippy::multiple_crate_versions)]

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use std::io::Result;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(json!({"ready": true,"message": "waiting now for request"}))
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(move || App::new().service(hello))
        .bind("0.0.0.0:8080")?
        .server_hostname("eywa.otechdo.org")
        .run()
        .await
}
