// use rustbu::api::api_controller::*;
// use rustbu::api::api_gamestate::*;
use axum::{response::*, routing::get, Router};
use rustbu::{api::web_sockets::*, rules::game_hodler::GameHodler};

#[tokio::main]
async fn main() {
    //tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/ws", get(handler))
        .with_state(GameHodler::new());

    let listener = tokio::net::TcpListener::bind("localhost:4444")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

