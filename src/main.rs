use axum::{
    response::IntoResponse,
    routing::get,
    routing::post,
    Router, Form,
    extract::State
};
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use serde_json::Value;
use sqlx::{Pool, Postgres};
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use tower_http::cors::CorsLayer;
use tower::ServiceBuilder;
use std::sync::Arc;

mod db;
mod auth;
mod message;

#[derive(Clone)]
struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let db = db::init_db().await.unwrap();
    let app_state = Arc::new(AppState{ db });

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);
    //comment

    let app = Router::new()
        .route("/", get(hey_world))
        .route("/login", post(login_post))
        .with_state(app_state)
        .with_state(io)
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

async fn login_post(Form(login): Form<auth::LoginForm>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    "Hey World"
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
