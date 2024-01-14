#![allow(unused)]

use std::{collections::{hash_map, HashMap}, hash::Hash};

use super::game_board::{Color, Board, Tile};
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc};

//TODO: Somehow authenticate players?
pub struct Player
{
    id: i8,
    color: Color,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game
{
    boards: [Board; 4],
    turn: Color,
}

#[derive(Debug)]
pub struct GameHodler
{
   pub games: Arc<Mutex<HashMap<i32, Game>>>
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
        let board_bb = Board::new_board(Color::Black, Color::Black);

        let board_ww = Board::new_board(Color::White, Color::White);
        let board_wb = Board::new_board(Color::White, Color::Black);

        return Game{boards: [board_bw, board_bb, board_wb, board_ww], turn: Color::Black};
    }

    pub fn next_turn(&mut self)
    {
        match self.turn
        {
            Color::White => self.turn = Color::Black,
            
            Color::Black => self.turn = Color::White,
        }
    }

    pub fn get_boards(&self) -> [Board; 4]
    {
        return self.boards;
    }
}

impl GameHodler
{
    pub fn default () -> GameHodler
    {
        return GameHodler{games: Arc::new(Mutex::new(HashMap::new()))}
    }

    pub fn add_game(&mut self, g: Game)
    {
        todo!()
    }
}