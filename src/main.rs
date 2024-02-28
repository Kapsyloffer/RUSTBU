use axum::{routing::get, Router};
use rustbu::{api::handle_socket::*, rules::game_hodler::GameHodler};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ws", get(handler))
        .with_state(GameHodler::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4444")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    //xd dick and balls
}

