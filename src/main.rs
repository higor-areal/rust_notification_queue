mod models;
mod queue;
mod workers;
mod state;

use std::sync::Arc;
use axum::{routing::get, Router};

use crate::{
    state::app_state::AppState,
    queue::channel::create_channel,
    workers::notification_worker::notification_worker,
};

#[tokio::main]
async fn main() {
    let (tx, rx) = create_channel();

    let state = AppState::new(tx);
    let shared = Arc::new(state);

    tokio::spawn(notification_worker(rx, shared.jobs.clone()));

    let app = Router::new()
        .route("/", get(|| async { "API Online" }))
        .with_state(shared);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Servidor rodando em http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}