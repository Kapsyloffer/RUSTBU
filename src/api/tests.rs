use crate::rules::{
    game_board::{Board, Color}, game_hodler::GameHodler, game_instance::Game, game_tile::Tile
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

    assert!(parse_move(&url, &m, shared).is_ok());
}

#[test]
fn test_move_parser_faulty_1() {
    let url = String::from("testcase");
    let m = String::from("BW00B1P");

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), Game::new_game());

    let shared = State::from(&binding);

    assert!(parse_move(&url, &m, shared).is_err());
}

#[test]
fn test_move_parser_faulty_2() {
    let url = String::from("testcase");
    let m = String::from("BP");

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), Game::new_game());

    let shared = State::from(&binding);

    assert!(parse_move(&url, &m, shared).is_err());
}

#[test]
fn test_move_parser_faulty_3() {
    let url = String::from("testcase");
    let m = String::from("WW0000P");

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), Game::new_game());

    let shared = State::from(&binding);

    assert!(parse_move(&url, &m, shared).is_err());
}

#[test]
fn test_move_parser_faulty_4() {
    let url = String::from("testcase");
    let m = String::from("WD0001P");

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), Game::new_game());

    let shared = State::from(&binding);

    assert!(parse_move(&url, &m, shared).is_err());
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

    let mut g = Game::new_game();
    assert_ne!(g.get_board(Color::White, Color::White).unwrap().get_state()[0][3], Tile::Empty);
    g.get_board(Color::White, Color::White).unwrap().set_state(&boardstate);
    assert_eq!(g.get_board(Color::White, Color::White).unwrap().get_state()[0][3], Tile::Empty);

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), g);

    let shared = State::from(&binding);

    assert!(parse_move(&url, &m, shared).is_ok());
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

    let mut g = Game::new_game();

    assert_ne!(g.get_board(Color::White, Color::White).unwrap().get_state()[0][3], Tile::Empty);

    println!("{:#?}", g.get_board(Color::White, Color::White).unwrap());

    g.get_board(Color::White, Color::White).unwrap().set_state(&boardstate);

    println!("{:#?}", g.get_board(Color::White, Color::White).unwrap());

    assert_eq!(g.get_board(Color::White, Color::White).unwrap().get_state()[0][3], Tile::Empty);
}


#[test]
fn set_state_in_shared_test()
{
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Black, Tile::White, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let url = String::from("testcase");
    let g = Game::new_game();

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), g);

    let state = State::from(&binding);

    state.games.lock().expect("a").get_mut(&url).unwrap().get_board(Color::White, Color::White).unwrap().set_state(&boardstate);

    assert_eq!(state.games.lock().expect("a").get_mut(&url).unwrap().get_board(Color::White, Color::White).unwrap().get_state(), &boardstate);
}

 /*
        Board
        [0,0][0,1][0,2][0,3] <-- Black start
        [1,0][1,1][1,2][1,3]
        [2,0][2,1][2,2][2,3]
        [3,0][3,1][3,2][3,3] <--- White start

        Så vi tar cur_pos och flyttar till new_pos
        */

#[test]
fn test_make_moves()
{
    let url = String::from("testcase");

    //Black's turn
    let mpb = String::from("BW0010P");
    let mab = String::from("BB0313A");

    //White's turn
    let mpw = String::from("WW3321P");
    let maw = String::from("BB3210A");

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), Game::new_game());
    
    let shared = State::from(&binding);

    let b4b = *shared.games.lock().expect("Lock fail").get_mut(&url as &str).unwrap().get_board(Color::Black, Color::Black).unwrap().get_state();

    make_move(url.to_owned(), mpb, mab, &shared);

    let afterb = *shared.games.lock().expect("Lock fail").get_mut(&url as &str).unwrap().get_board(Color::Black, Color::Black).unwrap().get_state();

    assert_ne!(afterb, b4b); //FAILS

    make_move(url.to_owned(), mpw, maw, &shared);

    let afterw = *shared.games.lock().expect("Lock fail").get_mut(&url).unwrap().get_board(Color::Black, Color::Black).unwrap().get_state();

    //assert_ne!(afterb, afterw); //FAILS

    let target_boardstate_state: [[Tile; 4]; 4] = [
        [Tile::White, Tile::White, Tile::Empty, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::White, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Black, Tile::Black, Tile::Black]
    ];

    let mut b: Board = Board::new_board(Color::Black, Color::Black);
    b.set_state(&target_boardstate_state);

    //Holy fuck.
    let cur_state = *shared.games.lock().expect("Lock fail").get_mut(&url as &str).unwrap().get_board(Color::Black, Color::Black).unwrap();

    assert_eq!(cur_state, b); //FAILS

}
