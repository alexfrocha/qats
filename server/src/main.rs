pub mod models;
pub mod services;
pub mod controllers;
pub mod test;
pub mod db;

use axum::routing::{ get, post, put, delete };
use axum::Router;
use controllers::sale_controller::{create_sale, get_sales, get_sales_by_buyer_id, get_sales_by_id, get_sales_by_seller_id, update_sale};
use controllers::station_controller::{create_station, delete_station, get_station_by_id, get_stations, update_station};
use controllers::store_controller::{create_store, delete_store, get_store_by_id, get_stores, update_store};
use controllers::user_controller::{ delete_user, get_user_by_cpf, get_user_by_email, get_user_by_id, get_users, post_user, update_user };


#[macro_use]
extern crate serde_derive;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async {"testing"}))
        .nest("/sales", sales_router())
        .nest("/stations", stations_router())
        .nest("/stores", stores_router())
        .nest("/users", users_router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn sales_router() -> Router {
    Router::new()
        .route("/", get(get_sales))
        .route("/create", post(create_sale))
        .route("/:id", get(get_sales_by_id))
        .route("/buyer/:buyer_id", get(get_sales_by_buyer_id))
        .route("/seller/:seller_id", get(get_sales_by_seller_id))
        .route("/update/:id", put(update_sale))
        .route("/delete/:id", delete(delete_station))
}

fn stations_router() -> Router {
    Router::new()
        .route("/", get(get_stations))
        .route("/create", post(create_station))
        .route("/:id", get(get_station_by_id))
        .route("/update/:id", put(update_station))
        .route("/delete/:id", delete(delete_station))
}

fn stores_router() -> Router {
    Router::new()
        .route("/", get(get_stores))
        .route("/create", post(create_store))
        .route("/:id", get(get_store_by_id))
        .route("/update/:id", put(update_store))
        .route("/delete/:id", delete(delete_store))
}

fn users_router() -> Router {
    Router::new()
        .route("/", get(get_users))
        .route("/create", post(post_user))
        .route("/id/:id", get(get_user_by_id))
        .route("/email/:email", get(get_user_by_email))
        .route("/cpf/:cpf", get(get_user_by_cpf))
        .route("/update/:id", put(update_user))
        .route("/delete/:id", delete(delete_user))
}