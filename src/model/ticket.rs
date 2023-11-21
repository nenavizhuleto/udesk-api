use crate::ModelController;
use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
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

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_str = match self {
            Status::Created => "created",
            Status::Assigned => "assigned",
            Status::Accepted => "accepted",
            Status::Confirm => "confirm",
            Status::Done => "done",
        };

        write!(f, "{status_str}")
    }
}

impl From<String> for Status {
    fn from(value: String) -> Self {
        match value.as_str() {
            "created" => Self::Created,
            "assigned" => Self::Assigned,
            "accepted" => Self::Accepted,
            "confirm" => Self::Confirm,
            "done" => Self::Done,
            _ => unreachable!("unknown status"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Ticket {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub user_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct TicketForCreate {
    pub title: String,
    pub description: String,
}

#[cfg(feature = "mysql")]
impl ModelController {
    pub async fn create_ticket(&self, user_id: i32, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let status = Status::Created.to_string();
        let created_at = chrono::Utc::now();
        let row = sqlx::query!(
            "INSERT INTO tickets (user_id, title, description, status, created_at) VALUES (?, ?, ?, ?, ?)",
            user_id,
            &ticket_fc.title,
            &ticket_fc.description,
            &status,
            &created_at,
        )
        .execute(&self.db)
        .await?;

        let ticket = Ticket {
            id: row.last_insert_id() as i32,
            user_id,
            title: ticket_fc.title,
            description: ticket_fc.description,
            status: Status::Created,
            created_at,
        };

        Ok(ticket)
    }
    pub async fn get_ticket(&self, id: i32) -> Result<Ticket> {
        let ticket = sqlx::query_as!(Ticket, "SELECT * FROM tickets WHERE id = ?", id)
            .fetch_one(&self.db)
            .await?;
        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let tickets = sqlx::query_as!(Ticket, "SELECT * FROM tickets")
            .fetch_all(&self.db)
            .await?;
        Ok(tickets)
    }

    pub async fn list_tickets_by_user(&self, user_id: i32) -> Result<Vec<Ticket>> {
        let tickets = sqlx::query_as!(Ticket, "SELECT * FROM tickets WHERE user_id = ?", user_id)
            .fetch_all(&self.db)
            .await?;
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: i32) -> Result<i32> {
        let result = sqlx::query!("DELETE FROM tickets WHERE id = ?", id)
            .execute(&self.db)
            .await?;
        Ok(result.rows_affected() as i32)
    }
}

#[cfg(feature = "memory")]
impl ModelController {
    pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            title: ticket_fc.title,
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();
        let tickets = store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());
        ticket.ok_or(Error::DeleteIdNotFound { id })
    }
}
