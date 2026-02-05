mod rate_limiter;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use rate_limiter::RateLimiterMiddleware;
use redis::aio::ConnectionManager;

// Application state to share Redis connection
struct AppState {
    redis: ConnectionManager,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Actix Web Server!")
}

#[get("/ratelimit")]
async fn ratelimit() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Hello, World!",
        "status": "success"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8081;
    println!("Starting server at http://127.0.0.1:{port}");

    // Connect to Redis
    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    println!("Connecting to Redis at: {}", redis_url);

    let client = redis::Client::open(redis_url).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Redis client error: {}", e),
        )
    })?;

    let redis_manager = ConnectionManager::new(client).await.map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Redis connection error: {}", e),
        )
    })?;

    println!("Successfully connected to Redis");

    // Create rate limiter middleware (10 requests per 60 seconds)

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                redis: redis_manager.clone(),
            }))
            .wrap(RateLimiterMiddleware)
            .service(index)
            .service(ratelimit)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
