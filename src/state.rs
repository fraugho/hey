use sqlx::{Pool, Postgres};
use dashmap::DashMap;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub session_store: DashMap<String, UserSession>, // Stores session ID -> User session mapping
}

#[derive(Clone)]
pub struct UserSession {
    pub username: String,
}
