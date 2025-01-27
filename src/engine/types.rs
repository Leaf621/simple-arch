use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Seller {
    pub id: Uuid,
    pub username: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Good {
    pub id: Uuid,
    pub name: String,
    pub price: u64,
    pub seller_id: Uuid,
    pub created_at: DateTime<Utc>,
}

