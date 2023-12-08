use axum::extract::Path;
use axum::routing::get;
use axum::Router;
use axum::{extract::State, Json};
use tracing::debug;

use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route(
            "/users/employees/:id/tickets",
            get(list_employee_tickets).post(create_employee_ticket),
        )
        .route("/users/employees/tickets", get(list_tickets))
        .route(
            "/users/employees/tickets/:id",
            get(get_ticket).delete(delete_ticket),
        )
        .with_state(mc)
}

async fn list_employee_tickets(
    State(mc): State<ModelController>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<Ticket>>> {
    debug!("{:<12} - list_employee_ticket", "HANDLER");

    let tickets = mc.list_tickets_by_user(id).await?;
    Ok(Json(tickets))
}

async fn create_employee_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<i64>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    debug!("{:<12} - create_employee_ticket", "HANDLER");

    let ticket = mc.create_ticket(id, ticket_fc).await?;
    Ok(Json(ticket))
}

async fn get_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<i64>,
) -> Result<Json<Ticket>> {
    debug!("{:<12} - get_ticket", "HANDLER");

    let ticket = mc.get_ticket(id).await?;
    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    debug!("{:<12} - list_ticket", "HANDLER");

    let tickets = mc.list_tickets().await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<i64>,
) -> Result<Json<i64>> {
    debug!("{:<12} - delete_ticket", "HANDLER");

    let rows_affected = mc.delete_ticket(id).await?;
    Ok(Json(rows_affected))
}
