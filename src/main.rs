pub use self::error::{Error, Result};
mod api;
mod error;
mod model;

use std::{env, net::SocketAddr, str::FromStr};

use axum::{middleware, response::Response, routing::get, Json, Router};
use model::ModelController;
use serde_json::json;
use sqlx::MySqlPool;
use tower_cookies::CookieManagerLayer;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let db = MySqlPool::connect(
        &env::var("DATABASE_URL").expect("DATABASE_URL not found in ENVIRONMENT"),
    )
    .await
    .map_err(|e| Error::DatabaseConnect {
        message: e.to_string(),
    })?;

    let mc = ModelController::new(db).await?;
    let routes_apis = api::tickets(mc.clone())
        .merge(api::users(mc.clone()))
        .merge(api::companies(mc.clone()))
        .route_layer(middleware::from_fn(api::auth_mw));

    let app = Router::new()
        .route("/", get(|| async { Json(json!({ "version": 1 })) }))
        .merge(api::auth())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(response_mapper))
        .layer(CookieManagerLayer::new());

    // TODO: Get from .env
    let addr = "127.0.0.1:3000";
    let addr = SocketAddr::from_str(addr).map_err(|e| Error::InvalidSocketAddr {
        message: e.to_string(),
    })?;

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| Error::BindServerError {
            message: e.to_string(),
        })?;

    Ok(())
}

async fn response_mapper(res: Response) -> Response {
    println!("INFO: {:<12} - response_mapper", "RES_MAPPER");

    println!();
    res
}
