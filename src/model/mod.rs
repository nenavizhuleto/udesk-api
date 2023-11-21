mod company;
mod department;
mod ticket;
mod user;

use crate::Result;

pub use company::*;
pub use department::*;
use sqlx::MySqlPool;
pub use ticket::*;
pub use user::*;

#[derive(Clone)]
pub struct ModelController {
    db: MySqlPool,
}

impl ModelController {
    pub async fn new(db: MySqlPool) -> Result<Self> {
        Ok(Self { db })
    }
}
