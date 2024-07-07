use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Station {
    id: Option<String>,
    name: String,
    description: Option<String>,
    location_addr: String,
    location_neighborhood: String,
    location_postal_code: String,
    location_city: String,
    location_state: String,
    location_lat: f32,
    location_lng: f32,
    images: Vec<String>,
    fuels: Vec<HashMap<String, String>>
} 