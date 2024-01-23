use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::*,
};
use serde::{Deserialize, Serialize};
use crate::{api::do_move::{do_move, Action}, rules::game_board::Color};

use crate::rules::{
    game_hodler::GameHodler, game_instance::Game,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
enum GamePacket {
    Action {
        id: String,
        move_p: Action,
        move_a: Action,
    },
    FetchMoves{
        url: String,
        h: Color,
        c: Color,
        x: i8,
        y: i8,
        aggr: bool,
    },
    CreateGame,
    GameCreated {
        id: String,
    },
    FetchGame {
        url: String,
    },
    NewState {
        board: String,
    },
    Exists{
        url: String,
    }
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
                    //Fetcha game efter id, then we do move
                    let mut games = game_hodler.games.lock().unwrap();
                    let Some(game) = games.get_mut(&id) else {return};
                    do_move(game, &move_p, &move_a);
                }
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
                GamePacket::Exists {url}=> {
                    let e: bool; //exists
                    let games = game_hodler.games.lock().unwrap().clone();
                    e = games.get(&url).is_some();

                    if socket.send(Message::Text(format!("{}", e))).await.is_err() {
                        return; 
                    }
                }
                //Send current gamestate
                GamePacket::FetchGame { url } => {
                    let mut games = game_hodler.games.lock().unwrap().clone();
                    let Some(game) = games.get_mut(&url) else {
                        return;
                    };
                    let state: String = serde_json::to_string(game).unwrap();
                    if socket.send(Message::Text(state)).await.is_err() {
                        return;
                    }
                }
                GamePacket::FetchMoves { url, h, c, x, y, aggr } => (),
                GamePacket::GameCreated { id } => {
                    if socket.send(Message::Text(id)).await.is_err() {
                        return;
                    }
                }
                GamePacket::NewState { board: _ } => {
                    todo!()
                }
            }
        }
    }
}