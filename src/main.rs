// use rustbu::api::api_controller::*;
// use rustbu::api::api_gamestate::*;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use rustbu::rules::{
    game_board::Color, game_hodler::GameHodler, game_instance::Game, game_tile::Tile,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
enum GamePacket {
    Action { id: String, action: Action },
    CreateGame { name: String },
    GameCreated { id: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Action {
    board: Color,
    home_side: Color,
    start_x: i8,
    start_y: i8,
    end_x: i8,
    end_y: i8,
    aggressive: bool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/ws", get(handler))
        .with_state(GameHodler::new());

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    Html(include_str!("../html/index.html"))
}

async fn handler(ws: WebSocketUpgrade, State(state): State<GameHodler>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, game_hodler: GameHodler) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            return;
        };

        if let Message::Text(text) = msg {
            println!("{}", text);
            let packet = match serde_json::from_str::<GamePacket>(&text) {
                Ok(packet) => packet,
                Err(e) => {
                    eprintln!("{e}");
                    return;
                }
            };

            match packet {
                GamePacket::CreateGame { name: _name } => {
                    let id = Game::generate_url();
                    game_hodler
                        .games
                        .lock()
                        .unwrap()
                        .insert(id.clone(), Game::new_game());

                    let packet = GamePacket::GameCreated { id };
                    if socket
                        .send(Message::Text(serde_json::to_string(&packet).unwrap()))
                        .await
                        .is_err()
                    {
                        return;
                    }
                }
                GamePacket::Action { id, action } => {
                    let mut games = game_hodler.games.lock().unwrap();
                    let Some(game) = games.get_mut(&id) else {
                        return;
                    };

                    let board = game.get_board(action.home_side, action.board).unwrap();

                    let moved = Tile::passive_move(
                        board,
                        (action.start_x, action.start_y),
                        (action.end_x, action.end_y),
                    );
                    println!("moved: {moved}");
                }
                GamePacket::GameCreated { .. } => (),
            }
        }
    }
}
