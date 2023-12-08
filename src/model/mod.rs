mod company;
mod department;
mod ticket;
mod user;

use crate::Result;

pub use company::*;
pub use department::*;
use sqlx::PgPool;
pub use ticket::*;
pub use user::*;

#[derive(Clone)]
pub struct ModelController {
    db: PgPool,
}

impl ModelController {
    pub async fn new(db: PgPool) -> Result<Self> {
        Ok(Self { db })
    }
}
