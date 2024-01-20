use crate::rules::{
    game_board::{Board, Color}, game_hodler::GameHodler, game_instance::Game, game_tile::Tile
};

use rocket::State;

use crate::api::api_gamestate::*;

#[test]
fn test_state() 
{
    let url = String::from("testcase");

    let binding = GameHodler::new();
    let shared = State::from(&binding);

    let mut games_map = shared.games.lock().expect("FAILED TO LOCK");

    assert!(games_map.len() == 0);

    games_map.insert(url.clone(), Game::new_game());

    assert!(games_map.len() == 1);
}

#[test]
fn test_move_parser() 
{
    let url = String::from("testcase");
    let m = String::from("BW0011P");

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), Game::new_game());

    let shared = State::from(&binding);

    assert!(parse_move(&url, &m, shared).is_ok());
}

#[test]
fn test_move_parser_faulty_1() 
{
    let url = String::from("testcase");
    let m = String::from("BW00B1P");

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), Game::new_game());

    let shared = State::from(&binding);

    assert!(parse_move(&url, &m, shared).is_err());
}

#[test]
fn test_move_parser_faulty_2() 
{
    let url = String::from("testcase");
    let m = String::from("BP");

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), Game::new_game());

    let shared = State::from(&binding);

    assert!(parse_move(&url, &m, shared).is_err());
}

#[test]
fn test_move_parser_faulty_3() 
{
    let url = String::from("testcase");
    let m = String::from("WW0000P");

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), Game::new_game());

    let shared = State::from(&binding);

    assert!(parse_move(&url, &m, shared).is_err());
}

#[test]
fn test_move_parser_faulty_4() 
{
    let url = String::from("testcase");
    let m = String::from("WD0001P");

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), Game::new_game());

    let shared = State::from(&binding);

    assert!(parse_move(&url, &m, shared).is_err());
}


#[test]
fn test_move_parser_faulty_5() 
{
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
        [0,0][0,1][0,2][0,3] <--- White start
        [1,0][1,1][1,2][1,3]
        [2,0][2,1][2,2][2,3]
        [3,0][3,1][3,2][3,3] <--- Black start

        Så vi tar cur_pos och flyttar till new_pos
*/


#[test]
fn test_make_moves_black()
{
    let url = String::from("testcase");

    //Black's turn
    let mpb = String::from("BW0010P"); // 0,0 -> 1,0
    let mab = String::from("BB0313A"); // 0,3 -> 1,3

    let binding = GameHodler::new();
    let mut g = Game::new_game();

    let b4_bw = *g.get_board(Color::Black, Color::White).unwrap();
    let b4_bb = *g.get_board(Color::Black, Color::Black).unwrap();

    binding.games.lock().expect("nah").insert(String::from(&url), g);

    let mut shared = State::from(&binding);

    make_move(String::from(&url), mpb, mab, &mut shared);

    let mut binding = shared.games.lock().expect("fail");
    let new_bw = binding.get_mut(&url as &str).unwrap().get_board(Color::Black, Color::White).unwrap().to_owned();
    let new_bb = binding.get_mut(&url as &str).unwrap().get_board(Color::Black, Color::Black).unwrap().to_owned();
    
    assert_ne!(b4_bw.get_state(), new_bw.get_state());
    assert_ne!(b4_bb.get_state(), new_bb.get_state());


}

#[test]
fn test_move_rocks()
{
    let url = String::from("testcase");
    let maw = String::from("BB3210A");

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), Game::new_game());
    
    let mut shared = State::from(&binding);

    let mut game_instance = shared.games.lock().expect("failed to lock").get_mut(&url).unwrap().clone();

    let b4 = game_instance.get_board(Color::Black, Color::Black).unwrap().clone();

    if move_rocks(&url, &maw, &mut shared).is_err()
    {
        panic!("Move_rocks is err.")
    }

    let after = shared.games.lock().expect("failed to lock").get_mut("testcase").unwrap().get_board(Color::Black, Color::Black).unwrap().clone();
    
    assert_ne!(b4.get_state(), after.get_state());
}


#[test]
fn replace_hashmap_in_shared_test()
{
    let url = String::from("testcase");

    let binding = GameHodler::new();
    binding.games.lock().expect("nah").insert(url.clone(), Game::new_game());
    
    let shared = State::from(&binding);

    let mut game_instance = shared.games.lock().expect("a").get_mut(&url).unwrap().clone();

    let b4 = game_instance.get_board(Color::Black, Color::White).unwrap().get_state().clone();

    let target_boardstate_state_bb: [[Tile; 4]; 4] = [
        [Tile::White, Tile::White, Tile::White, Tile::White],
        [Tile::Black, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Black, Tile::Black, Tile::Empty, Tile::Black]
    ];

    
    let after_board: &mut Board = game_instance.get_board(Color::Black, Color::White).unwrap();
    after_board.set_state(&target_boardstate_state_bb);

    shared.games.lock().expect("a").insert(url, game_instance.clone());

    let after = shared.games.lock().expect("a").get_mut("testcase").unwrap().get_board(Color::Black, Color::White).unwrap().get_state().clone();

    assert_ne!(b4, after);
}