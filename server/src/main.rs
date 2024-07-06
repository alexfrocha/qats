pub mod models;
pub mod services;
pub mod controllers;
pub mod db;

use axum::routing::{ get, post, put, delete };
use axum::Router;
use controllers::user_controller::{ delete_user, get_users, post_user, update_user };
use db::set_database;


#[macro_use]
extern crate serde_derive;

#[tokio::main]
async fn main() {
    // Tenta configurar o banco de dados, com retentativa em caso de falha inicial
    set_database().await.unwrap();

    // Configura as rotas
    let app = Router::new()
        .route(
            "/",
            get(|| async { "hello world" })
        )
        .route("/users", get(get_users))
        .route("/users/create", post(post_user))
        .route("/users/update/:id", put(update_user))
        .route("/users/delete/:id", delete(delete_user));

    // Inicia o servidor HTTP
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
