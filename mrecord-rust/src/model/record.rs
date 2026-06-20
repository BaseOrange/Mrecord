use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateRecord {
    pub title: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal,
}

#[derive(Serialize)]
pub struct RecordResponse {
    pub id: i32,
    pub title: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal,
    pub created_at: String,
}

impl From<crate::entity::record::Model> for RecordResponse {
    fn from(m: crate::entity::record::Model) -> Self {
        Self {
            id: m.id,
            title: m.title,
            amount: m.amount,
            created_at: m.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}
