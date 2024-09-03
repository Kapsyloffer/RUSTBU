use axum::
{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},response::*,
};
use crate::
{
    api::{game_handling::{create_game, fetch_game, get_all_games}, move_handling::*},
    rules::game_hodler::GameHodler,
};

use super::{game_handling::{fetch_previous_moves, join_game}, game_packets::GamePacket};

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
                //recieve and process movement action.
                GamePacket::Action { url, move_p, move_a } => {
                    do_move( &game_hodler, &url, &move_p, &move_a).await;
                }
                //Create a new game.
                GamePacket::CreateGame {player_id, color} => {
                    create_game(&mut socket, player_id, &color, false, &game_hodler).await;
                }
                GamePacket::CreateGameWithAI { player_id, color } => {
                    create_game(&mut socket, player_id, &color, true, &game_hodler).await;
                }
                //Response on create a new game.
                GamePacket::GameCreated { url } => {
                    if socket.send(Message::Text(url)).await.is_err() {
                        return;
                    }
                }
                //Send current gamestate.
                GamePacket::FetchGame { url } => {
                    fetch_game(&mut socket, &url, &game_hodler).await;
                }
                //Request to join a game.
                GamePacket::JoinGame { url , player_id} => {
                    join_game(&mut socket, &url,  &player_id, &game_hodler).await;
                }
                //Get possible moves.
                GamePacket::FetchMoves { url, h, c, x, y, aggr, player} => {
                    fetch_moves(&mut socket, &game_hodler, &url, &h, &c, &x, &y, &aggr, &player).await;
                }
                //Previously made moves.
                GamePacket::FetchPreviousMoves { url } => {
                    fetch_previous_moves(&mut socket, &game_hodler, &url).await;
                }
                //Previously made moves.
                GamePacket::GetAllGames => {
                    //TODO: Make json, and return
                    get_all_games(&mut socket, &game_hodler).await;
                }
                _ => unimplemented!(),
            }
        }
    }
}