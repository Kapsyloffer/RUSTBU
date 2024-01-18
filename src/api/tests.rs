use crate::rules::{
    game_board::Color, game_hodler::GameHodler, game_instance::Game, game_tile::Tile
};

use rocket::State;

use crate::api::api_gamestate::*;

#[test]
fn test_state() {
    let url = String::from("testcase");

    let binding = GameHodler::new();
    let shared = State::from(&binding);

    let mut games_map = shared.games.lock().expect("FAILED TO LOCK");

    assert!(games_map.len() == 0);

    games_map.insert(url.clone(), Game::new_game());

    assert!(games_map.len() == 1);
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

#[test]
fn test_move_parser_faulty_1() {
    let url = String::from("testcase");
    let m = String::from("BW00B1P");

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), Game::new_game());

    let shared = State::from(&binding);

    assert!(!parse_move(&url, &m, shared));
}

#[test]
fn test_move_parser_faulty_2() {
    let url = String::from("testcase");
    let m = String::from("BP");

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), Game::new_game());

    let shared = State::from(&binding);

    assert!(!parse_move(&url, &m, shared));
}

#[test]
fn test_move_parser_faulty_3() {
    let url = String::from("testcase");
    let m = String::from("WW0000P");

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), Game::new_game());

    let shared = State::from(&binding);

    assert!(!parse_move(&url, &m, shared));
}

#[test]
fn test_move_parser_faulty_4() {
    let url = String::from("testcase");
    let m = String::from("WD0001P");

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), Game::new_game());

    let shared = State::from(&binding);

    assert!(!parse_move(&url, &m, shared));
}


#[test]
fn test_move_parser_faulty_5() {
    let url = String::from("testcase");
    let m = String::from("WW0100A");

    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Black, Tile::White, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let g = Game::new_game();
    assert_ne!(g.get_board(Color::White, Color::White).unwrap().get_state()[0][3], Tile::Empty);
    g.get_board(Color::White, Color::White).unwrap().set_state(&boardstate);
    assert_eq!(g.get_board(Color::White, Color::White).unwrap().get_state()[0][3], Tile::Empty);

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), g);

    let shared = State::from(&binding);

    assert!(parse_move(&url, &m, shared));
}

#[test]
fn set_state_test()
{
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Black, Tile::White, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let g = Game::new_game();

    assert_ne!(g.get_board(Color::White, Color::White).unwrap().get_state()[0][3], Tile::Empty);

    println!("{:#?}", g.get_board(Color::White, Color::White).unwrap());

    g.get_board(Color::White, Color::White).unwrap().set_state(&boardstate);

    println!("{:#?}", g.get_board(Color::White, Color::White).unwrap());

    assert_eq!(g.get_board(Color::White, Color::White).unwrap().get_state()[0][3], Tile::Empty);
}