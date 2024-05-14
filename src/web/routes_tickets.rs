use crate::ctx::Ctx;
use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;
use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};
use serde_json::Value;
use tower_cookies::Cookies;

pub fn routes(model_controller: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(model_controller)
}

async fn create_ticket(
    State(model_controller): State<ModelController>,
    ctx: Ctx,
    Json(ticket_for_create): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<16} - create_ticket", "handler");

    let ticket = model_controller
        .create_ticket(ctx, ticket_for_create)
        .await?;

    Ok(Json(ticket))
}

async fn list_tickets(
    State(model_controller): State<ModelController>,
    ctx: Ctx,
) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<16} - list_tickets", "handler");

    let tickets = model_controller.list_tickets(ctx).await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(model_controller): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<16} - delete_ticket", "handler");

    let ticket = model_controller.delete_ticket(ctx, id).await?;

    Ok(Json(ticket))
}
