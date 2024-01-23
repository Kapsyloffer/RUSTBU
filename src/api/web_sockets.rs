use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade}, State
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
    },
    NewState{
        board: String,
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
                //Send current gamestate
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
                        .is_err(){
                            return;
                        }
                }
                //recieve and process movement action
                GamePacket::Action { id, move_p, move_a } => {
                    let mut games = game_hodler.games.lock().unwrap();
                    let Some(game) = games.get_mut(&id) else {
                        return;
                    };

                    //Make move on p
                    let mut board_p = game
                        .get_board(move_p.home_colour, move_p.board_colour)
                        .unwrap();
                    let b4_p = board_p.clone(); //In case it breaks
                    let moved_p: bool =
                        Tile::passive_move(board_p, (move_p.x1, move_p.y1), (move_p.x2, move_p.y2));
                    println!("moved_p: {moved_p}");

                    //Make move on a
                    let mut board_a = game
                        .get_board(move_a.home_colour, move_a.board_colour)
                        .unwrap();
                    let b4_a = board_a.clone();  //In case it breaks
                    let moved_a: bool =
                        Tile::passive_move(board_a, (move_a.x1, move_a.y1), (move_a.x2, move_a.y2));
                    println!("moved_a: {moved_a}");

                    //If either move fail.
                    if !moved_p || !moved_a
                    {
                        //Reset passive move board
                        game
                        .get_board(move_p.home_colour, move_p.board_colour)
                        .unwrap()
                        .set_state(b4_p.get_state());

                        //Reset aggressive move board
                        game
                        .get_board(move_a.home_colour, move_a.board_colour)
                        .unwrap()
                        .set_state(b4_a.get_state());
                        
                        return;
                    }else{
                        game.next_turn();
                    }

                    let new_state = GamePacket::NewState {
                        board: String::from(serde_json::to_string(&game).unwrap()),
                    };

                }
                GamePacket::GameCreated { id } => {
                    if socket
                        .send(Message::Text(id))
                        .await
                        .is_err(){
                            return;
                        }
                },
                GamePacket::NewState { board } => {},
            }
        }
    }
}
