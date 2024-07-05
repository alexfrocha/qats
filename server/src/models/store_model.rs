#[derive(Serialize, Deserialize)]
pub struct Store {
    id: Option<String>,
    name: String,
    description: String,
    location_addr: String,
    location_neighborhood: String,
    location_postal_code: String,
    location_city: String,
    location_state: String,
    location_lat: f32,
    location_lng: f32,
    images: Vec<String>
} 