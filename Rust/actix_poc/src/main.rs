use actix_web::{get, post, put, App, HttpResponse, HttpServer, Responder};

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
    HttpServer::new(|| App::new().service(get_post))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
