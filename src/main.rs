use axum::{
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    http::StatusCode,
    Router, Form,
    extract::State,
};
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use serde_json::Value;
use sqlx::Pool;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use tower_http::cors::CorsLayer;
use tower::ServiceBuilder;
use std::sync::Arc;
use uuid::Uuid;
use dashmap::DashMap;
use axum::http::header;
use axum_extra::extract::cookie::{Cookie};
use auth::*;

mod db;
mod auth;
mod message;
mod state;

use crate::state::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let db = db::init_db().await.unwrap();

    let app_state = Arc::new(AppState {
        db,
        session_store: DashMap::new(),
    });

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    let app = Router::new()
        .route("/", get(hey_world))
        .route("/login", post(login_post))
        .with_state(app_state)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn hey_world() -> impl IntoResponse {
    "Hey World"
}

async fn login_post(
    State(state): State<Arc<AppState>>,
    Form(login): Form<auth::LoginForm>,
) -> impl IntoResponse {
    match auth::check_login(&login, state.clone()).await {
        Ok(user_session) => {
            // Generate a session ID and store the session
            let session_id = Uuid::new_v4().to_string();
            state.session_store.insert(session_id.clone(), user_session);

            // Set session cookie
            let cookie = format!("session_id={}; HttpOnly; SameSite=Strict; Path=/", session_id);

            // Create a response and insert the Set-Cookie header
            let mut response = Redirect::to("/dashboard").into_response();
            response.headers_mut().insert(header::SET_COOKIE, cookie.parse().unwrap());

            response
        }
        Err(LoginError::InvalidCredentials) => {
            // Handle invalid credentials, possibly returning an error response
            (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
        }
        _ => {
            // Handle other potential errors
            (StatusCode::INTERNAL_SERVER_ERROR, "An error occurred").into_response()
        }
    }
}

async fn login_get() -> impl IntoResponse {
    //send to static html and js from next js
    "Hey World"
}

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);
    socket.on("message", |_socket: SocketRef, Data::<Value>(data)| {
        info!("Received message: {:?}", data);
    });
}
