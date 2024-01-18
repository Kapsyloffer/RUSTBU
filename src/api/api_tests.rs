use crate::rules::game_state::
{
    Game, 
    GameHodler
};

use rocket::State;

use crate::api::api_gamestate::*;

#[test]
fn test_state() {
    let url = String::from("testcase");

    let binding = GameHodler::new();
    let shared = State::from(&binding);

    let mut games_map = shared.games.lock().expect("FAILED TO LOCK");
    games_map.insert(url.clone(), Game::new_game());

    assert!(games_map.len() > 0);
}

#[test]
fn test_move_parser() {
    let url = String::from("testcase");
    let m = String::from("BW0011P");

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), Game::new_game());

    let shared = State::from(&binding);
    
    assert!(parse_move(&url, &m, shared));
}