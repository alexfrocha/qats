pub struct Request {
    pub user_id: String,
    pub station_id: String,
    pub status: String
}

impl Request {
    pub fn new_empty() -> Self {
        Self {
            user_id: String::new(),
            station_id: String::new(),
            status: String::new(),
        }
    }
}