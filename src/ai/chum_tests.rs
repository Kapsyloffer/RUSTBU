use crate::rules::{game_board::Board, game_tile::Tile};
use crate::rules::game_instance::Game;
use crate::api::move_handling::MovementAction;
use super::chum_bucket::ChumBucket;

#[test]
fn test_get_rock_positions_1() {
    let state: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::White, Tile::Empty, Tile::Empty, Tile::Empty],
    ];
    let mut board = Board::new_board(Tile::Black, Tile::White);
    board.set_state(&state);

    let mut target_w: Vec<(i8, i8)> = Vec::new();
    target_w.push((3, 0));

    let mut target_b: Vec<(i8, i8)> = Vec::new();
    target_b.push((0, 3));

    assert_eq!(ChumBucket::get_rock_positions(&board, Tile::White), target_w);
    assert_eq!(ChumBucket::get_rock_positions(&board, Tile::Black), target_b);
}

#[test]
fn test_get_rock_positions_2() {
    let state: [[Tile; 4]; 4] = [
        [Tile::White, Tile::White, Tile::White, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Black, Tile::Black, Tile::Black, Tile::Black],
    ];
    let mut board = Board::new_board(Tile::Black, Tile::White);
    board.set_state(&state);

    let mut target_w: Vec<(i8, i8)> = Vec::new();
    target_w.push((0, 0));
    target_w.push((0, 1));
    target_w.push((0, 2));
    target_w.push((0, 3));

    let mut target_b: Vec<(i8, i8)> = Vec::new();
    target_b.push((3, 0));
    target_b.push((3, 1));
    target_b.push((3, 2));
    target_b.push((3, 3));

    assert_eq!(ChumBucket::get_rock_positions(&board, Tile::White), target_w);
    assert_eq!(ChumBucket::get_rock_positions(&board, Tile::Black), target_b);
}

#[test]
fn test_ai_1() {
    let state_ww: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
        [Tile::Black, Tile::Black, Tile::Black, Tile::Black],
    ];

    let mut g = Game::new_game();
    g.get_board(Tile::White, Tile::White).unwrap().set_state(&state_ww);

    let mut ai = ChumBucket::new();
    let best_moves = ai.get_move(&mut g, Tile::Black);
    println!("\n{:#?}\n", best_moves);

    let target_move = MovementAction::new(Tile::White, Tile::White, 2, 3, 3, 2, true, String::from("ChumBucketAI"));

    assert_eq!(best_moves.1, target_move);
}