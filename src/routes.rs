use axum::{
    routing::get,
    Router,
};

use crate::handlers::tasks;
use crate::states::AppState;

pub fn app(state: AppState)-> Router{
    Router::new()
        .route("/health", get(tasks::health))
        .route("/tasks", get(tasks::list).post(tasks::create))
        .route("/tasks/{id}", get(tasks::get_one).delete(tasks::delete).patch(tasks::update))
        .with_state(state)
}

