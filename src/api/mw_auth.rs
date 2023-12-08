use std::collections::BTreeMap;

use axum::{
    http::{HeaderMap, Request},
    middleware::Next,
    response::Response,
};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use tracing::debug;

use crate::{config::config, Error, Result};

pub async fn mw_require_auth<B>(
    headers: HeaderMap,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    debug!("{:<12} - mw_require_auth", "MIDDLEWARE");
    let config = config();
    if !headers.contains_key(&config.TOKEN_HEADER) {
        debug!("{:<12} - mw_require_auth - headers doesn't contain token", "MIDDLEWARE");
        return Err(crate::Error::AuthFail);
    }
    let auth_token = headers.get(&config.TOKEN_HEADER).unwrap();

    let key: Hmac<Sha256> = Hmac::new_from_slice(&config.JWT_SECRET.clone().into_bytes())
        .map_err(|_| Error::AuthTokenVerifyError)?;
    let token_str = auth_token
        .to_str()
        .map_err(|_| Error::AuthTokenVerifyError)?;
    let claims: BTreeMap<String, String> = token_str
        .verify_with_key(&key)
        .map_err(|_| Error::AuthTokenVerifyError)?;

    // FIXME: Actually validate token
    if claims["access"] != "access" {
        debug!("{:<12} - mw_require_auth - access denied: {:?}", "MIDDLEWARE", claims["access"]);
        return Err(crate::Error::AuthFail);
    }

    Ok(next.run(req).await)
}
