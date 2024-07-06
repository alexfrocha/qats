use axum::{ http::StatusCode, Json };
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{ db::pool, models::user_model::User, services::user_service::{create_user_in_db, get_all_users_in_db} };

pub async fn get_users() -> (StatusCode, Json<Vec<User>>) {
    let shared_pool = pool().await;
    match get_all_users_in_db(&shared_pool).await {
        Ok(users) => (StatusCode::OK, Json(users)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    }
}

pub async fn post_user(
    Json(mut user_data): Json<Value>,
) -> (StatusCode, Json<User>) {
    let shared_pool = pool().await;
    let uuid = Uuid::now_v7();
    user_data["id"] = json!(uuid.to_string());

    match serde_json::from_value::<User>(user_data.clone()) {
        Ok(user) => {
            match create_user_in_db(&shared_pool, &user).await {
                Ok(_) => {
                    println!("Usuário criado com sucesso: {:?}", user_data["id"]);
                    (StatusCode::CREATED, Json(user))
                }
                Err(_) => {
                    println!("Erro ao criar usuário no banco de dados");
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(User::new_empty()))
                }
            }
        }
        Err(_) => {
            println!("Dados inválidos recebidos");
            (StatusCode::BAD_REQUEST, Json(User::new_empty()))
        }
    }
}
