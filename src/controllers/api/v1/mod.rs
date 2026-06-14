use crate::app_state::AppState;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new()
}
