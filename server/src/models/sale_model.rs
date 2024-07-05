use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
pub struct Sale {
    id: Option<String>,
    created_at: SystemTime,
    seller_id: String,
    buyer_id: String,
    status: String,
    info_currency: String,
    info_place: String,
    info_amount: String,
    info_price: String
}