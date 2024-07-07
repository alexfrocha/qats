pub mod models;
pub mod services;
pub mod controllers;
pub mod test;
pub mod db;

use axum::routing::{ get, post, put, delete };
use axum::Router;
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
        // '/store' route
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
