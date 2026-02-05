mod fixed_window;
mod sliding_window_counter;

use actix_web::{
    Error, HttpResponse,
    body::BoxBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web,
};
use futures_util::future::LocalBoxFuture;
use path_tree::PathTree;
use redis::{AsyncCommands, aio::ConnectionManager};

use std::future::{Ready, ready};
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{AppState, rate_limiter::fixed_window::FixedWindow};
use lazy_static::lazy_static;

lazy_static! {
    static ref RATE_LIMITED_ROUTES: PathTree<String> = {
        let mut tree = PathTree::new();
        tree.insert("/user/{user_id}", "user_id".to_string());
        tree.insert("/merchant/{merchant_id}", "merchant_id".to_string());
        tree.insert("", "__GLOBAL__".to_string());
        tree
    };
}
pub trait RateLimiterStrategy {
    async fn rate_limit(self, redis: &mut ConnectionManager, key: &str) -> Result<bool, ()>;
    fn build_key(key: &str, window: &str) -> String {
        format!("rate_limit:{key}:{window}")
    }
}
pub struct RateLimit<S>
where
    S: RateLimiterStrategy,
{
    pub strategy: S,
    pub id: String,
}

// Rate Limiter Middleware
#[derive(Clone)]
pub struct RateLimiterMiddleware;

impl<S> Transform<S, ServiceRequest> for RateLimiterMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct RateLimiterMiddlewareService<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for RateLimiterMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Get client IP address
            let mut redis = match req.app_data::<web::Data<AppState>>() {
                Some(data) => data.redis.clone(), // Clone the manager for async use
                None => {
                    // This is a critical error: AppState was not configured.
                    eprintln!("RateLimiter Error: AppState not found.");
                    let response = HttpResponse::InternalServerError().body("Configuration Error");
                    return Ok(req.into_response(response));
                }
            };

            let x = FixedWindow {
                window_size: 60,
                max_limit: 4,
            };
            let rate_lim = x.rate_limit(&mut redis, req.path()).await;

            match rate_lim {
                Ok(true) => service.call(req).await,
                Ok(false) => {
                    let response = HttpResponse::TooManyRequests().body("Configuration Error");
                    return Ok(req.into_response(response));
                }
                Err(err) => {
                    println!("error occured  while rate limited{:?}", err);
                    service.call(req).await
                }
            }

            // Continue with the request
        })
    }
}
