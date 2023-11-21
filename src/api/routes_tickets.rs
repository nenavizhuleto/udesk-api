use axum::extract::Path;
use axum::routing::get;
use axum::Router;
use axum::{extract::State, Json};

use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", get(list_tickets).post(create_ticket))
        .route("/tickets/:id", get(get_ticket).delete(delete_ticket))
        .with_state(mc)
}

async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("INFO: {:<12} - create_ticket", "HANDLER");
    println!("INFO: {:<12} - create_ticket: {:?}", "PAYLOAD", ticket_fc);

    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}

async fn get_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<i32>,
) -> Result<Json<Ticket>> {
    println!("INFO: {:<12} - get_ticket", "HANDLER");
    let ticket = mc.get_ticket(id).await?;
    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("INFO: {:<12} - list_ticket", "HANDLER");

    let tickets = mc.list_tickets().await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<i32>,
) -> Result<Json<i32>> {
    println!("INFO: {:<12} - delete_ticket", "HANDLER");

    let rows_affected = mc.delete_ticket(id).await?;

    Ok(Json(rows_affected))
}
