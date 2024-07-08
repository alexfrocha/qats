use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{db::pool, models::store_model::Store, services::store_service::{create_store_in_db, delete_store_in_db_by_id, get_all_stores_in_db, get_store_in_db_by_id, update_store_in_db}};

pub async fn get_stores() -> impl IntoResponse {
    let shared_pool = pool().await;
    match get_all_stores_in_db(&shared_pool).await {
        Ok(stores) => (StatusCode::OK, Json(stores)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
    }
}

pub async fn get_store_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let shared_pool = pool().await;
    match get_store_in_db_by_id(&shared_pool, &id).await {
        Ok(store) => (StatusCode::OK, Json(store)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
    }
}

pub async fn create_store(Json(mut store_data): Json<Value>) -> impl IntoResponse {
    let shared_pool = pool().await;
    let uuid = Uuid::now_v7();
    store_data["id"] = json!(uuid.to_string());
    match serde_json::from_value::<Store>(store_data.clone()) {
        Ok(store) => {
            match create_store_in_db(&shared_pool, &store).await {
                Ok(_) => (StatusCode::CREATED, Json(store)),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Store::new_empty()))
            }
        }
        Err(_) => (StatusCode::BAD_REQUEST, Json(Store::new_empty()))
    }
}

pub async fn update_store(Path(id): Path<String>, Json(store_data): Json<Value>) -> impl IntoResponse {
    let shared_pool = pool().await;
    match serde_json::from_value::<Store>(store_data.clone()) {
        Ok(store) => {
            match update_store_in_db(&shared_pool, &id, &store).await {
                Ok(_) => (StatusCode::OK, Json(store)),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Store::new_empty()))
            }
        },
        Err(_) => (StatusCode::BAD_REQUEST, Json(Store::new_empty()))
    }
}

pub async fn delete_store(Path(id): Path<String>) -> impl IntoResponse {
    let shared_pool = pool().await;
    match delete_store_in_db_by_id(&shared_pool, &id).await {
        Ok(_) => (StatusCode::OK, Json(json!("deleted".to_string()))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!("not deleted")))
    }
}