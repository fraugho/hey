use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        ConnectInfo,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use serde_json::Value;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use tower_http::cors::CorsLayer;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);
    //comment

    let app = Router::new()
        .route("/", get(hey_world))
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

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);
    socket.on("message", |_socket: SocketRef, Data::<Value>(data)| {
        info!("Received message: {:?}", data);
    });
}
