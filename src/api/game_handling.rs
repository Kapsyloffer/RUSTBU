use axum::extract::ws::WebSocket;
use axum::extract::ws::Message;
use crate::api::game_packets::*;
use crate::rules::game_hodler::GameHodler; 
use crate::rules::game_instance::Game;
use crate::rules::game_tile::Tile;


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

pub async fn create_game(socket: &mut WebSocket, player_id: String, color: &Tile, game_hodler: &GameHodler) {
    //Just to prevent collissions (Rare af but yaknow, just in case.)
    let map_size = game_hodler.games.lock().unwrap().len();
    let url = format!("{}{}", Game::generate_url(), map_size);
    println!("\n{} created game: {}\n", player_id, url);
    game_hodler
        .games
        .lock()
        .unwrap()
        .insert(url.to_owned(), Game::new_game());

    game_hodler.games.lock().unwrap().get_mut(&url).unwrap().add_player(player_id, Some(*color));

    let packet = GamePacket::GameCreated { url };
    if socket
        .send(Message::Text(serde_json::to_string(&packet).unwrap()))
        .await
        .is_err() {
        return;
    }
}

pub async fn join_game(_socket: &mut WebSocket, url: &String, player_id: &String, game_hodler: &GameHodler){
    let mut binding = game_hodler
    .games
    .lock()
    .unwrap();

    if binding.get_mut(url).unwrap().add_player(player_id.to_owned(), None){
        println!("{} added to: {}!", player_id.to_owned(), url);
    }else {
        println!("{} not added to: {}!", player_id.to_owned(), url);
    }
}

pub async fn fetch_previous_moves(socket: &mut WebSocket, game_hodler: &GameHodler, url: &String) {
    let moves_tuple = game_hodler.moves.lock().unwrap().get(url).cloned();
    //Just in case there are no moves previously made (Start of the game.)
    if let Some(moves_tuple) = moves_tuple{
        let packet = GamePacket::PreviousMoves{move_p: moves_tuple.0, move_a: moves_tuple.1};
        let serialized = serde_json::to_string(&packet);

        if let Ok(serialized) = serialized {
            let serialized_message = Message::Text(serialized.clone());
            
            if socket.send(serialized_message).await.is_err() {
                return;
            }
        } else {
            eprintln!("Error serializing moves_tuple");
        }   
    }
}