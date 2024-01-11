#![allow(unused)]

use crate::rules::game_rules::{Color, Board, Tile};

//TODO: Somehow authenticate players?
pub struct Player
{
    id: i8,
    color: Color,
}

pub struct Game
{
    boards: [Board; 4],
    turn: Color,
}

impl Player 
{
    pub fn new_white() -> Player
    {
        return Player {id: 123, color: Color::White};
    }

    pub fn new_black() -> Player
    {
        return Player {id: 123, color: Color::Black};
    }
}

impl Game
{
    pub fn new_game () -> Game
    {
        let board_bw = Board::new_board(Color::Black, Color::White);
        let board_ww = Board::new_board(Color::White, Color::White);

        let board_bb = Board::new_board(Color::Black, Color::Black);
        let board_wb = Board::new_board(Color::White, Color::Black);

        return Game{boards: [board_bw, board_ww, board_bb, board_wb], turn: Color::Black};
    }
}