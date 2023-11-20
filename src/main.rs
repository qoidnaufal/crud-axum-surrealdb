use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{
    routing::{delete, get, patch, post},
    Router,
};

mod db;
mod errors;
mod models;
mod routes;

use db::Database;
use routes::AppState;

// ---- main

#[tokio::main]
async fn main() {
    let db = Database::init()
        .await
        .expect("error connecting to database");

    let app = Router::new()
        .route("/get", get(routes::get_orders))
        .route("/get/:uuid", get(routes::get_pizza))
        .route("/post", post(routes::order_pizza))
        .route("/update/:uuid", patch(routes::update_order))
        .route("/delete", delete(routes::delete_pizza))
        .with_state(Arc::new(AppState { db: db.clone() }));

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server failed to start");
}
