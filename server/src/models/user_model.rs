
#[derive(Serialize, Deserialize)]
pub struct User {
    id: Option<String>,
    name: String,
    email: String,
    password: String,
    date_of_birth: String,
    cpf: String,
    location_lat: f32,
    location_lng: f32,
    uniques_store: String,
    uniques_station: String,
    uniques_can_change: bool,
    role: String,
    phone_number: String
}