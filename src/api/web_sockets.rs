use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade}, Path, State
    },
    response::*,
};
use serde::{Deserialize, Serialize};

use crate::rules::{
    game_board::Color, game_hodler::GameHodler, game_instance::Game, game_tile::Tile,
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
    GameCreated {
        id: String,
    },
    FetchGame{
        url: String
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Action {
    board_colour: Color,
    home_colour: Color,
    x1: i8,
    y1: i8,
    x2: i8,
    y2: i8,
    aggr: bool,
}

pub async fn handler(ws: WebSocketUpgrade, State(state): State<GameHodler>) -> Response {
    return ws.on_upgrade(|socket| handle_socket(socket, state));
}

pub async fn handle_socket(mut socket: WebSocket, game_hodler: GameHodler) {
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
                GamePacket::CreateGame => {
                    let id = Game::generate_url();
                    println!("\nCreated game: {}\n", id);
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
                GamePacket::FetchGame { url } =>
                {
                    let mut games = game_hodler.games.lock().unwrap().clone();
                    let Some(game) = games.get_mut(&url) else {
                        return;
                    };
                    let state: String = serde_json::to_string(game).unwrap();
                    if socket
                        .send(Message::Text(state))
                        .await
                        .is_err()
                    {
                        return;
                    }
                }
                GamePacket::Action { id, move_p, move_a } => {
                    let mut games = game_hodler.games.lock().unwrap();
                    let Some(game) = games.get_mut(&id) else {
                        return;
                    };

                    let board_p = game
                        .get_board(move_p.home_colour, move_p.board_colour)
                        .unwrap();
                    let moved_p: bool =
                        Tile::passive_move(board_p, (move_p.x1, move_p.y1), (move_p.x2, move_p.y2));
                    println!("moved_p: {moved_p}");

                    let board_a = game
                        .get_board(move_a.home_colour, move_a.board_colour)
                        .unwrap();
                    let moved_a: bool =
                        Tile::passive_move(board_a, (move_a.x1, move_a.y1), (move_a.x2, move_a.y2));
                    println!("moved_a: {moved_a}");

                    if moved_p && moved_a {
                        game.next_turn();
                    }
                    //println!("{:#?}", game);
                    
                    //DEBUG
                    let size = games.len();

                    println!("{}", size);
                }
                GamePacket::GameCreated { .. } => (),
            }
        }
    }
}
