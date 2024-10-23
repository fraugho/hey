use axum::{
    response::{IntoResponse, Redirect, Response, Html},
    routing::{get, get_service, post},
    http::{StatusCode, Method, HeaderValue},
    Router, Form,
    extract::State,
    middleware,
    Json,
};
use std::{path::PathBuf, fs};
use tower_http::services::ServeDir;
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
use axum_extra::extract::cookie::Cookie;
use auth::*;
mod db;
mod auth;
mod message;
mod state;
use crate::state::*;

async fn serve_html_file(path: &str) -> impl IntoResponse {
    let html_content = fs::read_to_string(format!("frontend/out/{}", path))
        .unwrap_or_else(|_| "404 Not Found".to_string());
    Html(html_content)
}

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
    
    // Configure CORS
    let cors = CorsLayer::new()
        // Allow specific origin instead of any
        .allow_origin("http://127.0.0.1:8080".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_credentials(true)
        .allow_headers([
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::AUTHORIZATION,
        ]);

    let static_files_service = ServeDir::new("frontend/out");
    let protected_routes = Router::new()
        .route("/dashboard", get(|| serve_html_file("dashboard.html")))
        .route("/", get(|| serve_html_file("index.html")))
        .route("/api/protected", get(protected_route))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_auth
        ));

    let app = Router::new()
        .route("/login", get(|| serve_html_file("login.html")))
        .route("/api/login", post(login_post))
        .merge(protected_routes)
        .fallback_service(static_files_service)
        .with_state(app_state)
        .layer(
            ServiceBuilder::new()
                .layer(cors)  // Use our configured CORS layer
                .layer(layer),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn protected_route() -> impl IntoResponse {
    "This is a protected route"
}

async fn login_post(
    State(state): State<Arc<AppState>>,
    Json(login): Json<auth::LoginForm>,
) -> impl IntoResponse {
    info!("login recieved");
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

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);
    socket.on("message", |_socket: SocketRef, Data::<Value>(data)| {
        info!("Received message: {:?}", data);
    });
}
