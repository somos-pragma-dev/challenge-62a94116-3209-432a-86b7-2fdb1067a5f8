use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub amount: f64,
    pub date: NaiveDateTime,
    pub transaction_type: String,
    pub status: String,
}