use axum::{body::Body, http::{Request, StatusCode}, routing::{delete, post, get, put}, Json, Router};
use serde_json::json;
use tower::ServiceExt;

use crate::{controllers::user_controller::{delete_user, get_user_by_cpf, get_user_by_email, get_user_by_id, get_users, post_user, update_user}, models::user_model::User};

// 01908ae9-5048-7490-b920-e9139525f25b

#[tokio::test]
async fn test_get_users() {
    let app = Router::new().route("/users", axum::routing::get(get_users));
    let response = app.oneshot(Request::builder().uri("/users").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_post_user() {
    let app = Router::new().route("/users/create", axum::routing::post(post_user));

    let new_user = json!({
        "active": false,
        "name": "álex",
        "email": "alexasdfasdfasdf2@gmail.com",
        "password": "123",
        "date_of_birth": "2006-08-10",
        "cpf": "123456",
        "location_lat": 2.0,
        "location_lng": 2.0,
        "uniques_store": "1asdadasdas",
        "uniques_station": "2dasdasdasdasdasdasd",
        "uniques_can_change": false,
        "role": "ADMIN",
        "phone_number": "71986758166"
    });

    let response = app.oneshot(Request::builder().uri("/users/create").method("POST").header("content-type", "application/json").body(Body::from(new_user.to_string())).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_update_user() {
    let app = Router::new().route("/users/update/:id", axum::routing::put(update_user));
    
    let new_user = json!({
        "active": true,
        "name": "fernando",
        "email": "alexasdfasdfasdf2@gmail.com",
        "password": "123",
        "date_of_birth": "2006-08-10",
        "cpf": "123456",
        "location_lat": 2.0,
        "location_lng": 2.0,
        "uniques_store": "1asdadasdas",
        "uniques_station": "2dasdasdasdasdasdasd",
        "uniques_can_change": false,
        "role": "ADMIN",
        "phone_number": "71986758166"
    });

    let response = app.oneshot(Request::builder().uri("/users/update/01908ae9-5048-7490-b920-e9139525f25b").method("PUT").header("content-type", "application/json").body(Body::from(new_user.to_string())).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_delete_user() {
    let app = Router::new().route("/users/delete/:id", axum::routing::delete(delete_user));
    let response = app.oneshot(Request::builder().uri("/users/delete/01908b02-820f-72a3-9366-8cd3158acf23").method("DELETE").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);    
}

#[tokio::test]
async fn test_get_user_by_id() {
    let app = Router::new().route("/users/id/:id", axum::routing::get(get_user_by_id));
    let response = app.oneshot(Request::builder().uri("/users/id/01908ae9-5048-7490-b920-e9139525f25b").method("GET").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_user_by_email() {
    let app = Router::new().route("/users/email/:email", axum::routing::get(get_user_by_email));
    let response = app.oneshot(Request::builder().uri("/users/email/alexasdfasdfasdf2@gmail.com").method("GET").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_user_by_cpf() {
    let app = Router::new().route("/users/cpf/:cpf", axum::routing::get(get_user_by_cpf));
    let response = app.oneshot(Request::builder().uri("/users/cpf/885815709506").method("GET").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}