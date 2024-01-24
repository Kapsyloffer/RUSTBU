use axum::extract::ws::WebSocket;
use axum::extract::ws::Message;
use crate::api::web_sockets::GamePacket;
use crate::rules::game_hodler::GameHodler; 
use crate::rules::game_instance::Game;


pub async fn fetch_game(socket: &mut WebSocket, url: &String, game_hodler: &GameHodler) {
    let mut games = game_hodler.games.lock().unwrap().to_owned();
    let Some(game) = games.get_mut(url) else {
        return;
    };
    let state: String = serde_json::to_string(game).unwrap();
    if socket.send(Message::Text(state)).await.is_err() {
        return;
    }
}

pub async fn create_game(socket: &mut WebSocket, game_hodler: &GameHodler) {
    let url = Game::generate_url();
    println!("\nCreated game: {}\n", url);
    game_hodler
        .games
        .lock()
        .unwrap()
        .insert(url.to_owned(), Game::new_game());

    let packet = GamePacket::GameCreated { url };
    if socket
        .send(Message::Text(serde_json::to_string(&packet).unwrap()))
        .await
        .is_err() {
        return;
    }
}

pub async fn check_exists(socket: &mut WebSocket, url: &String, game_hodler: &GameHodler) {
    let e: bool; //exists
    let games = game_hodler.games.lock().unwrap().to_owned();
    e = games.get(url).is_some();
    
    if socket.send(Message::Text(format!("{}", e))).await.is_err() {
        return;
    }
}
