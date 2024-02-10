use axum::extract::ws::WebSocket;
use axum::extract::ws::Message;
use crate::api::game_packets::*;
use crate::rules::game_hodler::GameHodler; 
use crate::rules::game_instance::Game;


pub async fn fetch_game(socket: &mut WebSocket, url: &String, game_hodler: &GameHodler) {
    let mut games = game_hodler.games.lock().unwrap().to_owned();
    let Some(game) = games.get_mut(url) else {
        return;
    };
    let state: String = format!("{:?}", game);
    let packet = GamePacket::FetchedGame { state };

    if socket
    .send(Message::Text(serde_json::to_string(&packet).unwrap()))
    .await
    .is_err() {
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
    let games = game_hodler.games.lock().unwrap().to_owned();
    let exists = games.get(url).is_some();
    
    if socket.send(Message::Text(format!("{}", exists))).await.is_err() {
        return;
    }
}

pub async fn join_game(_socket: &mut WebSocket, url: &String, player_id: &String, game_hodler: &GameHodler){
    let mut binding = game_hodler
    .games
    .lock()
    .unwrap();

    if binding.get_mut(url).unwrap().add_player(player_id.to_owned()){
        println!("{} added to: {}!", player_id.to_owned(), url);
    }else {
        println!("{} not added to: {}!", player_id.to_owned(), url);
    }
}

pub async fn fetch_previous_moves(socket: &mut WebSocket, game_hodler: &GameHodler, url: &String) {
    let moves_tuple = game_hodler.moves.lock().unwrap().get(url).unwrap().clone();
    let serialized = serde_json::to_string(&moves_tuple);

    if let Ok(serialized) = serialized {
        let serialized_message = Message::Text(serialized.clone());
        println!("{}", serialized);
        
        if socket.send(serialized_message).await.is_err() {
            return;
        }
    } else {
        // Handle serialization error
        eprintln!("Error serializing moves_tuple");
    }
}