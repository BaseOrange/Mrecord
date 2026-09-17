use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::common::money::{
    deserialize_decimal_from_number_or_string, serialize_decimal_as_number,
};

#[derive(Deserialize)]
pub struct CreateRecord {
    pub title: String,
    #[serde(deserialize_with = "deserialize_decimal_from_number_or_string")]
    pub amount: Decimal,
}

#[derive(Serialize)]
pub struct RecordResponse {
    pub id: i32,
    pub title: String,
    #[serde(serialize_with = "serialize_decimal_as_number")]
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
