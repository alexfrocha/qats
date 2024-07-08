use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{db::pool, models::station_model::Station, services::station_service::{create_station_in_db, delete_station_in_db_by_id, get_all_stations_in_db, get_station_in_db_by_id, update_station_in_db}};

pub async fn get_stations() -> impl IntoResponse {
    let shared_pool = pool().await;
    match get_all_stations_in_db(&shared_pool).await {
        Ok(stations) => (StatusCode::OK, Json(stations)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
    }
}

pub async fn get_station_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let shared_pool = pool().await;
    match get_station_in_db_by_id(&shared_pool, &id).await {
        Ok(station) => (StatusCode::OK, Json(station)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
    }
}

pub async fn create_station(Json(mut station_data): Json<Value>) -> impl IntoResponse {
    let shared_pool = pool().await;
    let uuid = Uuid::now_v7();
    station_data["id"] = json!(uuid.to_string());
    match serde_json::from_value::<Station>(station_data.clone()) {
        Ok(station) => {
            match create_station_in_db(&shared_pool, &station).await {
                Ok(_) => (StatusCode::CREATED, Json(station)),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Station::new_empty()))
            }
        },
        Err(_) => (StatusCode::BAD_REQUEST, Json(Station::new_empty()))
    }
}

pub async fn update_station(Path(id): Path<String>, Json(station_data): Json<Value>) -> impl IntoResponse {
    let shared_pool = pool().await;
    match serde_json::from_value::<Station>(station_data.clone()) {
        Ok(station) => {
            match update_station_in_db(&shared_pool,  &id, &station).await {
                Ok(_) => (StatusCode::CREATED, Json(station)),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Station::new_empty()))
            }
        },
        Err(_) => (StatusCode::BAD_REQUEST, Json(Station::new_empty()))
    }
}

pub async fn delete_station(Path(id): Path<String>) -> impl IntoResponse {
    let shared_pool = pool().await;
    match delete_station_in_db_by_id(&shared_pool, &id).await {
        Ok(_) => (StatusCode::OK, Json(json!("deleted"))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!("not deleted")))
    }
}