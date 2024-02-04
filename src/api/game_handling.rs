use axum::extract::ws::{Message, WebSocket};
use tokio::sync::broadcast;

use crate::{
    api::game_packets::*,
    rules::{
        game_hodler::{GameHodler, Lobby},
        game_instance::Game,
    },
};

pub async fn fetch_game(socket: &mut WebSocket, url: &String, game_hodler: &GameHodler) {
    let packet = {
        let mut games = game_hodler.games.lock().unwrap();
        let Some(game) = games.get_mut(url) else {
            return;
        };
        let state: String = format!("{:?}", game);
        GamePacket::FetchedGame { state }
    };

    //println!("{:#?}", packet);

    if socket
        .send(Message::Text(serde_json::to_string(&packet).unwrap()))
        .await
        .is_err()
    {
        return;
    }
}

pub async fn create_game(
    socket: &mut WebSocket,
    game_hodler: &GameHodler,
    sender: broadcast::Sender<()>,
) {
    let url = Game::generate_url();
    println!("\nCreated game: {}\n", url);
    game_hodler.games.lock().unwrap().insert(
        url.to_owned(),
        Lobby {
            game: Game::new_game(),
            sender,
        },
    );

    let packet = GamePacket::GameCreated { url };
    if socket
        .send(Message::Text(serde_json::to_string(&packet).unwrap()))
        .await
        .is_err()
    {
        return;
    }
}

pub async fn check_exists(socket: &mut WebSocket, url: &String, game_hodler: &GameHodler) {
    let exists = {
        let games = game_hodler.games.lock().unwrap();
        games.get(url).is_some()
    };

    if socket
        .send(Message::Text(format!("{}", exists)))
        .await
        .is_err()
    {
        return;
    }
}

pub async fn join_game(
    _socket: &mut WebSocket,
    url: &String,
    player_id: &String,
    game_hodler: &GameHodler,
) {
    if game_hodler
        .games
        .lock()
        .unwrap()
        .get_mut(url)
        .unwrap()
        .game
        .add_player(player_id.to_owned())
    {
        println!("Player added to: {}!", url);
    } else {
        println!("Player not added to: {}!", url);
    }
}
