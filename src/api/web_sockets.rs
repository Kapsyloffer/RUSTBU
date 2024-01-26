use axum::
{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade},State,},response::*,
};
use serde::
{
    Deserialize, Serialize
};
use crate::
{
    api::{game_handling::{check_exists, create_game, fetch_game}, move_handling::*},
    rules::{game_board::Color, game_hodler::GameHodler},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub (crate) enum GamePacket {
    Action { //Movement action
        url: String,
        move_p: Action,
        move_a: Action,
    },
    CreateGame, //Call to create new game.

    CheckExists { //Call to check if game exists
        url: String,
    },
    FetchGame { //Call to fetch game state
        url: String,
    },
    FetchMoves { //Call to fetch moves from a rock on a board
        url: String,
        h: Color,
        c: Color,
        x: i8,
        y: i8,
        aggr: bool,
    },
    FetchedMoves { //Response containing move positions for rock on the requested board.
        moves: String,
    },
    FetchedGame { //Response containing the requested gamestate.
        state: String,
    },
    GameCreated { //Response containing game url
        url: String,
    }
}

pub async fn handler(ws: WebSocketUpgrade, State(state): State<GameHodler>) -> Response {
    return ws.on_upgrade(|socket| handle_socket(socket, state));
}

pub async fn handle_socket(mut socket: WebSocket, game_hodler: GameHodler) {
    while let Some(m_res) = socket.recv().await {
        let msg = if let Ok(m_res) = m_res {
            m_res
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
                GamePacket::Action { url, move_p, move_a } => {
                    do_move(&game_hodler, &url, &move_p, &move_a).await;
                }
                GamePacket::CreateGame => {
                    create_game(&mut socket, &game_hodler).await;
                }
                GamePacket::CheckExists { url } => {
                    check_exists(&mut socket, &url, &game_hodler).await;
                }
                //Send current gamestate
                GamePacket::FetchGame { url } => {
                    fetch_game(&mut socket, &url, &game_hodler).await;
                }
                GamePacket::FetchMoves { url, h, c, x, y, aggr} => {
                    fetch_moves(&mut socket, &game_hodler, &url, &h, &c, &x, &y, &aggr).await;
                }
                GamePacket::GameCreated { url } => {
                    if socket.send(Message::Text(url)).await.is_err() {
                        return;
                    }
                }
                _ => unimplemented!(),
            }
        }
    }
}