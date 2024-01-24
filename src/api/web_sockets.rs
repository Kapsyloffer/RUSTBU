use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::*,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::do_move::{do_move, Action},
    rules::{game_board::Color, game_hodler::GameHodler, game_instance::Game, game_tile::Tile},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
enum GamePacket {
    Action {
        id: String,
        move_p: Action,
        move_a: Action,
    },
    CreateGame,
    Exists {
        url: String,
    },
    FetchGame {
        url: String,
    },
    FetchMoves {
        url: String,
        h: Color,
        c: Color,
        x: i8,
        y: i8,
        aggr: bool,
    },
    FetchedMoves {
        moves: String,
    },
    GameCreated {
        id: String,
    },
    NewState {
        board: String,
    },
}

pub async fn handler(ws: WebSocketUpgrade, State(state): State<GameHodler>) -> Response {
    return ws.on_upgrade(|socket| handle_socket(socket, state));
}

pub async fn handle_socket(mut socket: WebSocket, game_hodler: GameHodler) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            println!("Socket ded");
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
                //recieve and process movement action
                GamePacket::Action { id, move_p, move_a } => {
                    do_move(&game_hodler, &id, &move_p, &move_a).await;
                }
                GamePacket::CreateGame => {
                    create_game(&mut socket, &game_hodler).await;
                }
                GamePacket::Exists { url } => {
                    check_exists(&mut socket, &url, &game_hodler).await;
                }
                //Send current gamestate
                GamePacket::FetchGame { url } => {
                    fetch_game(&mut socket, &url, &game_hodler).await;
                }
                GamePacket::FetchMoves { url, h, c, x, y, aggr} => {
                    fetch_moves(&mut socket, &game_hodler, &url, &h, &c, &x, &y, &aggr).await;
                }
                GamePacket::GameCreated { id } => {
                    if socket.send(Message::Text(id)).await.is_err() {
                        return;
                    }
                }
                _ => unimplemented!(),
            }
        }
    }
}

async fn fetch_game(socket: &mut WebSocket, url: &String, game_hodler: &GameHodler) {
    let mut games = game_hodler.games.lock().unwrap().clone();
    let Some(game) = games.get_mut(url) else {
        return;
    };
    let state: String = serde_json::to_string(game).unwrap();
    if socket.send(Message::Text(state)).await.is_err() {
        return;
    }
}

async fn create_game(socket: &mut WebSocket, game_hodler: &GameHodler) {
    let id = Game::generate_url();
    println!("\nCreated game: {}\n", id);
    game_hodler
        .games
        .lock()
        .unwrap()
        .insert(id.clone(), Game::new_game());

    let mut g = Game::new_game();
    println!("{}", g.display());

    let packet = GamePacket::GameCreated { id };
    if socket
        .send(Message::Text(serde_json::to_string(&packet).unwrap()))
        .await
        .is_err()
    {
        return;
    }
}

async fn check_exists(socket: &mut WebSocket, url: &String, game_hodler: &GameHodler) {
    let e: bool; //exists
    let games = game_hodler.games.lock().unwrap().clone();
    e = games.get(url).is_some();

    if socket.send(Message::Text(format!("{}", e))).await.is_err() {
        return;
    }
}

async fn fetch_moves(socket: &mut WebSocket, game_hodler: &GameHodler, url: &String, h: &Color, c: &Color, x: &i8, y: &i8, aggr: &bool,) {
    let mut binding = game_hodler.games.lock().unwrap().clone();
    let b = binding.get_mut(url).unwrap().get_board(*h, *c).unwrap();

    let move_list = format!("{:?}", Tile::get_possible_moves(b, *aggr, (*x, *y)));
    println!("fetch_moves: {}", move_list);

    if socket.send(Message::Text(move_list)).await.is_err() {
        return;
    }
}
