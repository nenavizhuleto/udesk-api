use axum::extract::Path;
use axum::routing::get;
use axum::Router;
use axum::{extract::State, Json};
use tracing::debug;

use crate::model::{ModelController, Role, User, UserForCreate};
use crate::Result;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/users", get(list_users))
        .route("/users/:id", get(get_user).delete(delete_user))
        .route(
            "/users/employees",
            get(list_employees).post(create_employee),
        )
        .route(
            "/users/executors",
            get(list_executors).post(create_executor),
        )
        .route("/users/admins", get(list_admins).post(create_admin))
        .with_state(mc)
}

async fn create_employee(
    State(mc): State<ModelController>,
    Json(user_fc): Json<UserForCreate>,
) -> Result<Json<User>> {
    debug!("{:<12} - create_employee", "HANDLER");
    debug!("{:<12} - create_employee: {:?}", "PAYLOAD", user_fc);

    let user = mc.create_user(user_fc, Role::Employee).await?;
    Ok(Json(user))
}

async fn create_executor(
    State(mc): State<ModelController>,
    Json(user_fc): Json<UserForCreate>,
) -> Result<Json<User>> {
    debug!("{:<12} - create_executor", "HANDLER");
    debug!("{:<12} - create_executor: {:?}", "PAYLOAD", user_fc);

    let user = mc.create_user(user_fc, Role::Executor).await?;
    Ok(Json(user))
}

async fn create_admin(
    State(mc): State<ModelController>,
    Json(user_fc): Json<UserForCreate>,
) -> Result<Json<User>> {
    debug!("{:<12} - create_admin", "HANDLER");
    debug!("{:<12} - create_admin: {:?}", "PAYLOAD", user_fc);

    let user = mc.create_user(user_fc, Role::Admin).await?;
    Ok(Json(user))
}

async fn get_user(State(mc): State<ModelController>, Path(id): Path<i64>) -> Result<Json<User>> {
    debug!("{:<12} - get_user", "HANDLER");

    let user = mc.get_user(id).await?;
    Ok(Json(user))
}

async fn list_users(State(mc): State<ModelController>) -> Result<Json<Vec<User>>> {
    debug!("{:<12} - list_user", "HANDLER");

    let users = mc.list_users().await?;
    Ok(Json(users))
}

async fn list_employees(State(mc): State<ModelController>) -> Result<Json<Vec<User>>> {
    debug!("{:<12} - list_employees", "HANDLER");

    let users = mc.list_employees().await?;
    Ok(Json(users))
}
async fn list_executors(State(mc): State<ModelController>) -> Result<Json<Vec<User>>> {
    debug!("{:<12} - list_executors", "HANDLER");

    let users = mc.list_executors().await?;
    Ok(Json(users))
}

async fn list_admins(State(mc): State<ModelController>) -> Result<Json<Vec<User>>> {
    debug!("{:<12} - list_admins", "HANDLER");

    let users = mc.list_admins().await?;
    Ok(Json(users))
}

async fn delete_user(State(mc): State<ModelController>, Path(id): Path<i64>) -> Result<Json<i64>> {
    debug!("{:<12} - delete_user", "HANDLER");

    let rows_affected = mc.delete_user(id).await?;
    Ok(Json(rows_affected))
}
