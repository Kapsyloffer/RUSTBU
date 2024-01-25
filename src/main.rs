// use rustbu::api::api_controller::*;
// use rustbu::api::api_gamestate::*;
use axum::{response::*, routing::get, Router};
use rustbu::{api::web_sockets::*, rules::game_hodler::GameHodler};

#[tokio::main]
async fn main() {
    //tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/ws", get(handler))
        .route("/game/:id", get(fetch_game)) //Join a game using URL
        .with_state(GameHodler::new());

    let listener = tokio::net::TcpListener::bind("localhost:4444")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    Html(include_str!("../html/index.html"))
}

async fn fetch_game() -> impl IntoResponse {
    Html(include_str!("../html/game.html"))
}
