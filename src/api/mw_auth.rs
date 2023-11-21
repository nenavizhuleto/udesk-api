use std::collections::BTreeMap;

use axum::{
    http::{HeaderMap, Request},
    middleware::Next,
    response::Response,
};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;

use crate::api::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_require_auth<B>(
    headers: HeaderMap,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("INFO: {:<12} - mw_require_auth", "MIDDLEWARE");
    if !headers.contains_key(AUTH_TOKEN) {
        return Err(crate::Error::AuthFail);
    }
    let auth_token = headers.get(AUTH_TOKEN).unwrap();

    let key: Hmac<Sha256> =
        Hmac::new_from_slice(&crate::api::JWT_KEY).map_err(|_| Error::AuthTokenVerifyError)?;
    let token_str = auth_token
        .to_str()
        .map_err(|_| Error::AuthTokenVerifyError)?;
    let claims: BTreeMap<String, String> = token_str
        .verify_with_key(&key)
        .map_err(|_| Error::AuthTokenVerifyError)?;

    // FIXME: Actually validate token
    if claims["access"] != "access" {
        return Err(crate::Error::AuthFail);
    }

    Ok(next.run(req).await)
}
