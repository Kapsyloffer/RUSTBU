use axum::
{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},response::*,
};
use crate::
{
    api::{game_handling::{check_exists, create_game, fetch_game}, move_handling::*},
    rules::game_hodler::GameHodler,
};

use super::{game_handling::join_game, game_packets::GamePacket};

pub async fn handler(
    ws: WebSocketUpgrade, 
    State(state): State<GameHodler>) -> Response {
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
            //println!("{}", text);
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
                    do_move( &game_hodler, &url, &move_p, &move_a).await;
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
                GamePacket::JoinGame { url , player_id} => {
                    join_game(&mut socket, &url,  &player_id, &game_hodler).await;
                }
                GamePacket::FetchMoves { url, h, c, x, y, aggr, player} => {
                    fetch_moves(&mut socket, &game_hodler, &url, &h, &c, &x, &y, &aggr, &player).await;
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