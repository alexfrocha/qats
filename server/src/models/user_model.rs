#[derive(Serialize, Debug, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub active: bool,
    pub name: String,
    pub email: String,
    pub password: String,
    pub date_of_birth: String,
    pub cpf: String,
    pub location_lat: f32,
    pub location_lng: f32,
    pub uniques_store: Option<String>,
    pub uniques_station: Option<String>,
    pub uniques_can_change: bool,
    pub role: String,
    pub phone_number: Option<String>,
}

impl User {
    pub fn new_empty() -> Self {
        Self {
            id: None,
            active: false,
            name: String::new(),
            email: String::new(),
            password: String::new(),
            date_of_birth: String::new(),
            cpf: String::new(),
            location_lat: 0.0,
            location_lng: 0.0,
            uniques_store: None,
            uniques_station: None,
            uniques_can_change: false,
            role: String::new(),
            phone_number: None,
        }
    }
}
