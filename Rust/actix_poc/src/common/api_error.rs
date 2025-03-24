use actix_web::{HttpResponse, ResponseError};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Database connection failed")]
    DatabaseError,

    #[error("User not found")]
    NotFound,

    #[error("Invalid request: {0}")]
    BadRequest(String),

    #[error("Internal server error")]
    InternalError,
}

// Convert `ApiError` into HTTP responses
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::DatabaseError => HttpResponse::InternalServerError().json("Database error"),
            ApiError::NotFound => HttpResponse::NotFound().json("User not found"),
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().json(msg),
            ApiError::InternalError => {
                HttpResponse::InternalServerError().json("Internal server error")
            }
        }
    }
}
