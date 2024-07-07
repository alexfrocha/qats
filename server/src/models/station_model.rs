use serde_json::Value as JsonValue;

use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Station {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub location_addr: String,
    pub location_neighborhood: String,
    pub location_postal_code: String,
    pub location_city: String,
    pub location_state: String,
    pub location_lat: f32,
    pub location_lng: f32,
    pub images: Option<JsonValue>,
    pub fuels: Option<JsonValue>
} 

impl Station {
    pub fn new_empty() -> Self {
        Self {
            id: None,
            name: String::new(),
            description: None,
            location_addr: String::new(),
            location_neighborhood: String::new(),
            location_postal_code: String::new(),
            location_city: String::new(),
            location_state: String::new(),
            location_lng: 0.0,
            location_lat: 0.0,
            images: None,
            fuels: None

        }
    }
}