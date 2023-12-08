mod api;
mod config;
mod error;
mod model;

pub use self::error::{Error, Result};
use axum::middleware::Next;
use axum::routing::options;
pub use config::config;

// #[cfg(test)] /// Commented for early development
pub mod _dev_utils;

use axum::http::{HeaderMap, HeaderValue, Method, Request};
use axum::{middleware, response::Response, Router};
use model::ModelController;
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use tracing::{debug, info};
use tracing_subscriber::{self, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // -- FOR DEV ONLY
    _dev_utils::init_dev().await;

    let config = config();

    debug!("loaded config: {config:?}");

    let db = PgPool::connect(&config.DATABASE_URL)
        .await
        .map_err(|e| Error::DatabaseConnect {
            message: e.to_string(),
        })?;

    let mc = ModelController::new(db).await?;
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);
    let routes_apis = api::users(mc.clone())
        .merge(api::tickets(mc.clone()))
        .merge(api::companies(mc.clone()))
        .route_layer(middleware::from_fn(api::auth_mw))
        .route_layer(middleware::from_fn(cors_mapper));

    let app = Router::new()
        .merge(api::auth())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(response_mapper));

    info!("starting server on {:?}", config.ADDRESS);
    axum::Server::bind(&config.ADDRESS)
        .serve(app.into_make_service())
        .await
        .map_err(|e| Error::BindServerError {
            message: e.to_string(),
        })?;

    Ok(())
}

async fn cors_mapper<B>(headers: HeaderMap, req: Request<B>, next: Next<B>) -> Result<Response> {
    let mut res = next.run(req).await;

    let headers = res.headers_mut();
    headers.insert(
        "Access-Control-Allow-Origin",
        HeaderValue::from_str("*").unwrap(),
    );

    Ok(res)
}

async fn response_mapper(mut res: Response) -> Response {
    debug!("{:<12} - response_mapper", "RES_MAPPER");
    debug!(
        "{:<12} - response_mapper - headers: {:?}",
        "RES_MAPPER",
        res.headers()
    );

    let headers = res.headers_mut();
    headers.insert(
        "Access-Control-Allow-Origin",
        HeaderValue::from_str("*").unwrap(),
    );

    res
}
