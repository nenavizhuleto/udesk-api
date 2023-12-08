use crate::ModelController;
use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Debug, Clone, Serialize)]
#[sqlx(type_name = "ticket_status", rename_all = "lowercase")]
pub enum Status {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "assigned")]
    Assigned,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "confirm")]
    Confirm,
    #[serde(rename = "done")]
    Done,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize)]
pub struct Ticket {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub user_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct TicketForCreate {
    pub title: String,
    pub description: String,
}

impl ModelController {
    pub async fn create_ticket(&self, user_id: i64, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let status = Status::Created;
        let created_at = chrono::Utc::now();
        let result = sqlx::query(
            "INSERT INTO tickets (user_id, title, description, status, created_at) VALUES ($1, $2, $3, $4, $5)")
            .bind(user_id)
            .bind(&ticket_fc.title)
            .bind(&ticket_fc.description)
            .bind(&status)
            .bind(&created_at)
        .execute(&self.db)
        .await?;

        let ticket = Ticket {
            id: result.rows_affected() as i64,
            user_id,
            title: ticket_fc.title,
            description: ticket_fc.description,
            status: Status::Created,
            created_at,
        };

        Ok(ticket)
    }
    pub async fn get_ticket(&self, id: i64) -> Result<Ticket> {
        let ticket: Ticket = sqlx::query_as("SELECT * FROM tickets WHERE id = $1").bind(id)
            .fetch_one(&self.db)
            .await?;
        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let tickets: Vec<Ticket> = sqlx::query_as("SELECT * FROM tickets")
            .fetch_all(&self.db)
            .await?;
        Ok(tickets)
    }

    pub async fn list_tickets_by_user(&self, user_id: i64) -> Result<Vec<Ticket>> {
        let tickets: Vec<Ticket> = sqlx::query_as("SELECT * FROM tickets WHERE user_id = $1").bind(user_id)
            .fetch_all(&self.db)
            .await?;
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: i64) -> Result<i64> {
        let result = sqlx::query("DELETE FROM tickets WHERE id = $1").bind(id)
            .execute(&self.db)
            .await?;
        Ok(result.rows_affected() as i64)
    }
}
