use chrono::{NaiveDateTime};
use sqlx;

#[derive(Debug, sqlx::FromRow)]
pub struct DbEsSettlement {
    pub id: String,
    pub service_id: String,
    pub created_at: Option<NaiveDateTime>,
    pub quantity: f64,
    pub price: f64,
    pub amount: f64,
}