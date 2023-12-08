use std::collections::BTreeMap;

use crate::{config, Error, Result};
use axum::{routing::post, Json, Router};
use jwt::SignWithKey;
use serde::Deserialize;
use serde_json::{json, Value};

use hmac::{Hmac, Mac};
use sha2::Sha256;
use tracing::debug;

pub fn routes() -> Router {
    Router::new().route("/api/auth", post(api_auth))
}

pub async fn api_auth(payload: Json<AuthPayload>) -> Result<Json<Value>> {
    debug!("{:<12} - api_auth", "HANDLER");
    debug!("{:<12} - {:?}", "PAYLOAD", payload);

    // FIXME: store password hashes and so on.
    if payload.password != "password" {
        return Err(Error::AuthFail);
    }

    // FIXME: Actually we don't need cookie authorization
    // because this API would be used by another more specific API
    // so authorization must be done with HTTP headers

    let key: Hmac<Sha256> = Hmac::new_from_slice(&config().JWT_SECRET.clone().into_bytes())
        .map_err(|_| Error::AuthTokenSignError)?;
    let mut claims = BTreeMap::new();
    claims.insert("access", payload.access.clone());
    let token_str = claims
        .sign_with_key(&key)
        .map_err(|_| Error::AuthTokenSignError)?;

    let body = Json(json!({
        "result": {
            "success": true,
            "token": token_str,
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    access: String,
    password: String,
}
