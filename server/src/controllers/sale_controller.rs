use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{db::pool, models::{sale_model::Sale, store_model::Store}, services::{sale_service::{create_sale_in_db, delete_sale_in_db_by_id, get_sale_in_db_by_buyer_id, get_sale_in_db_by_id, get_sale_in_db_by_seller_id, update_sale_in_db}, store_service::get_all_stores_in_db}};

pub async fn get_sales() -> impl IntoResponse {
    let shared_pool = pool().await;
    match get_all_stores_in_db(&shared_pool).await {
        Ok(sales) => (StatusCode::OK, Json(sales)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
    }
}

pub async fn get_sales_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let shared_pool = pool().await;
    match get_sale_in_db_by_id(&shared_pool, &id).await {
        Ok(sale) => (StatusCode::OK, Json(sale)),
        Err(_) => (StatusCode::NOT_FOUND, Json(None))
    }
}

pub async fn get_sales_by_buyer_id(Path(buyer_id): Path<String>) -> impl IntoResponse {
    let shared_pool = pool().await;
    match get_sale_in_db_by_buyer_id(&shared_pool, &buyer_id).await {
        Ok(sale) => (StatusCode::OK, Json(sale)),
        Err(_) => (StatusCode::NOT_FOUND, Json(None))
    }
}

pub async fn get_sales_by_seller_id(Path(seller_id): Path<String>) -> impl IntoResponse {
    let shared_pool = pool().await;
    match get_sale_in_db_by_seller_id(&shared_pool, &seller_id).await {
        Ok(sale) => (StatusCode::OK, Json(sale)),
        Err(_) => (StatusCode::NOT_FOUND, Json(None))
    }
}

pub async fn create_sale(Json(mut sale_data): Json<Value>) -> impl IntoResponse {
    let shared_pool = pool().await;
    let uuid = Uuid::now_v7();
    sale_data["id"] = json!(uuid.to_string());
    match serde_json::from_value::<Sale>(sale_data.clone()) {
        Ok(sale) => {
            match create_sale_in_db(&shared_pool, &sale).await {
                Ok(_) => (StatusCode::OK, Json(sale)),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Sale::new_empty()))
            }
        },
        Err(_) => (StatusCode::BAD_REQUEST, Json(Sale::new_empty()))
    }
}


pub async fn update_sale(Path(id): Path<String>, Json(sale_data): Json<Value>) -> impl IntoResponse {
    let shared_pool = pool().await;
    match serde_json::from_value::<Sale>(sale_data.clone()) {
        Ok(sale) => {
            match update_sale_in_db(&shared_pool, &id, &sale).await {
                Ok(_) => (StatusCode::OK, Json(sale)),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Sale::new_empty()))
            }
        },
        Err(_) => (StatusCode::BAD_REQUEST, Json(Sale::new_empty()))
    }
}

pub async fn delete_sale(Path(id): Path<String>) -> impl IntoResponse {
    let shared_pool = pool().await;
    match delete_sale_in_db_by_id(&shared_pool, &id).await {
        Ok(_) => (StatusCode::OK, Json(json!("deleted"))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!("not deleted")))
    }
}