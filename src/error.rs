use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConfigMissingEnv(String),
    ConfigInvalidAddress(String),
    AuthFail,
    AuthTokenSignError,
    AuthTokenVerifyError,
    InvalidSocketAddr { message: String },
    DeleteIdNotFound { id: i32 },
    BindServerError { message: String },
    DatabaseConnect { message: String },
    Database { message: String },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("ERR: {:<12} - {self:?}", "INTO_RES");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("{self:?}")})),
        )
            .into_response()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        return Self::Database {
            message: value.to_string(),
        };
    }
}
