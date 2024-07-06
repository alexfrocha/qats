#[derive(Serialize, Debug, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub date_of_birth: String,
    pub cpf: String,
    pub location_lat: f32,
    pub location_lng: f32,
    pub uniques_store: String,
    pub uniques_station: String,
    pub uniques_can_change: bool,
    pub role: String,
    pub phone_number: String,
}

impl User {
    pub fn new_empty() -> Self {
        Self {
            id: None,
            name: String::new(),
            email: String::new(),
            password: String::new(),
            date_of_birth: String::new(),
            cpf: String::new(),
            location_lat: 0.0,
            location_lng: 0.0,
            uniques_store: String::new(),
            uniques_station: String::new(),
            uniques_can_change: false,
            role: String::new(),
            phone_number: String::new(),
        }
    }
}
