pub mod models;
pub mod services;
pub mod controllers;
pub mod test;
pub mod db;

use axum::routing::{ get, post, put, delete };
use axum::Router;
use controllers::station_controller::{create_station, delete_station, get_station_by_id, get_stations, update_station};
use controllers::store_controller::{create_store, delete_store, get_store_by_id, get_stores, update_store};
use controllers::user_controller::{ delete_user, get_user_by_cpf, get_user_by_email, get_user_by_id, get_users, post_user, update_user };


#[macro_use]
extern crate serde_derive;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route(
            "/",
            get(|| async { "hello world" })
        )
        // '/sale' route
        // '/station' route
        .route("/stations", get(get_stations))
        .route("/stations/create", post(create_station))
        .route("/stations/:id", get(get_station_by_id))
        .route("/stations/update/:id", put(update_station))
        .route("/stations/delete/:id", delete(delete_station))
        // '/store' route
        .route("/stores", get(get_stores))
        .route("/stores/create", post(create_store))
        .route("/stores/:id", get(get_store_by_id))
        .route("/stores/update/:id", put(update_store))
        .route("/stores/delete/:id", delete(delete_store))
        // '/users' route
        .route("/users", get(get_users))
        .route("/users/create", post(post_user))
        .route("/users/id/:id", get(get_user_by_id))
        .route("/users/email/:email", get(get_user_by_email))
        .route("/users/cpf/:cpf", get(get_user_by_cpf))
        .route("/users/update/:id", put(update_user))
        .route("/users/delete/:id", delete(delete_user));

    // Inicia o servidor HTTP
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
