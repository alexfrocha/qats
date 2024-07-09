use std::time::SystemTime;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow};


#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Sale {
    pub id: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub seller_id: Option<String>,
    pub buyer_id: String,
    pub status: String,
    pub info_currency: String,
    pub info_place: String,
    pub info_amount: Option<String>,
    pub info_price: Option<String>
}

impl Sale {
    pub fn new_empty() -> Self {
        Self {
            id: None,
            created_at: None,
            seller_id: None,
            buyer_id: String::new(),
            status: String::new(),
            info_currency: String::new(),
            info_place: String::new(),
            info_amount: None,
            info_price: None,
        }
    }
}