mod common;
mod config;
mod database;
mod db_models;
mod router;
mod schema;
mod services;
use actix_web::{get, post, put, App, HttpResponse, HttpServer, Responder};
use std::io;

#[get("/")]
async fn get_post() -> impl Responder {
    HttpResponse::Ok().body("hellow world")
}
#[post("/echo")]
async fn echo(request_body: String) -> impl Responder {
    HttpResponse::Ok().body("echo")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server running at 127.0.0.1:8080");
    let config = config::Config::new().map_err(|e| {
        eprintln!("Config error: {}", e);
        io::Error::new(io::ErrorKind::Other, e.to_string())
    })?;
    HttpServer::new(|| App::new().service(get_post))
        .bind((config.host, config.port))?
        .run()
        .await
}
