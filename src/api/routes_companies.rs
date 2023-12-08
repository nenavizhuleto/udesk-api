use axum::extract::Path;
use axum::routing::get;
use axum::Router;
use axum::{extract::State, Json};
use tracing::debug;

use crate::model::{Company, CompanyForCreate, Department, DepartmentForCreate, ModelController};
use crate::Result;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/companies", get(list_companies).post(create_company))
        .route("/companies/:id", get(get_company).delete(delete_company))
        .route(
            "/companies/:id/departments",
            get(list_company_departments).post(create_company_department),
        )
        .route(
            "/companies/departments/:id",
            get(get_company_department).delete(delete_company_department),
        )
        .with_state(mc)
}

async fn create_company(
    State(mc): State<ModelController>,
    Json(company_fc): Json<CompanyForCreate>,
) -> Result<Json<Company>> {
    debug!("{:<12} - create_company", "HANDLER");
    debug!("{:<12} - create_company: {:?}", "PAYLOAD", company_fc);

    let company = mc.create_company(company_fc).await?;

    Ok(Json(company))
}

async fn create_company_department(
    State(mc): State<ModelController>,
    Path(id): Path<i64>,
    Json(dep_fc): Json<DepartmentForCreate>,
) -> Result<Json<Department>> {
    debug!("{:<12} - create_company", "HANDLER");
    debug!("{:<12} - create_company: {:?}", "PAYLOAD", dep_fc);

    let dep = mc.create_department(id, dep_fc).await?;

    Ok(Json(dep))
}

async fn get_company(
    State(mc): State<ModelController>,
    Path(id): Path<i64>,
) -> Result<Json<Company>> {
    debug!("{:<12} - get_company", "HANDLER");
    let company = mc.get_company(id).await?;
    Ok(Json(company))
}

async fn list_company_departments(
    State(mc): State<ModelController>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<Department>>> {
    debug!("{:<12} - get_company", "HANDLER");
    let company = mc.list_departments_by_company(id).await?;
    Ok(Json(company))
}

async fn get_company_department(
    State(mc): State<ModelController>,
    Path(id): Path<i64>,
) -> Result<Json<Department>> {
    debug!("{:<12} - get_company", "HANDLER");
    let dep = mc.get_department(id).await?;
    Ok(Json(dep))
}

async fn list_companies(State(mc): State<ModelController>) -> Result<Json<Vec<Company>>> {
    debug!("{:<12} - list_company", "HANDLER");

    let companies = mc.list_companies().await?;

    Ok(Json(companies))
}

async fn delete_company(
    State(mc): State<ModelController>,
    Path(id): Path<i64>,
) -> Result<Json<i64>> {
    debug!("{:<12} - delete_company", "HANDLER");

    let rows_affected = mc.delete_company(id).await?;

    Ok(Json(rows_affected))
}

async fn delete_company_department(
    State(mc): State<ModelController>,
    Path(id): Path<i64>,
) -> Result<Json<i64>> {
    debug!("{:<12} - delete_company", "HANDLER");

    let rows_affected = mc.delete_department(id).await?;

    Ok(Json(rows_affected))
}
