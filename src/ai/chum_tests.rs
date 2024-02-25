use crate::rules::{game_board::Board, game_tile::Tile};

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