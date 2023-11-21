use crate::ModelController;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Ticket {
    pub id: i32,
    pub title: String,
    pub user_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct TicketForCreate {
    pub user_id: i32,
    pub title: String,
}

#[cfg(feature = "mysql")]
impl ModelController {
    pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let row = sqlx::query!(
            "INSERT INTO tickets (user_id, title) VALUES (?, ?)",
            &ticket_fc.user_id,
            &ticket_fc.title
        )
        .execute(&self.db)
        .await?;
        let id = row.last_insert_id() as i32;

        let ticket = Ticket {
            id,
            user_id: ticket_fc.user_id,
            title: ticket_fc.title,
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
