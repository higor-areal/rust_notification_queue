mod models;
mod queue;
mod workers;
mod state;
mod responses;
mod handlers;

use std::sync::Arc;
use axum::{routing::{get, post}, Router};

use crate::{
    state::app_state::AppState,
    queue::channel::create_channel,
    workers::notification_worker::notification_worker,
    handlers::notification_handler::{notifications, get_job, get_all_jobs}
};

#[tokio::main]
async fn main() {
    let (tx, rx) = create_channel();

    let state = AppState::new(tx);
    let shared = Arc::new(state);

    tokio::spawn(notification_worker(rx, shared.jobs.clone()));

    let app = Router::new()
        .route("/", get(|| async { "API Online" }))
        .route("/notifications", post(notifications))
        .route("/jobs/{uuid}", get(get_job))
        .route("/jobs", get(get_all_jobs))
        .with_state(shared);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Servidor rodando em http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}