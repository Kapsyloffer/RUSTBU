//Alterantive name: StupidFish

use crate::{api::move_handling::MovementAction, rules::{game_board::Board, game_instance::Game, game_tile::Tile}};

pub struct ChumBucket {
    best_move_p: Option<MovementAction>,
    best_move_a: Option<MovementAction>,
    best_rock_count: i8,
    best_range: i8 
}

impl ChumBucket {
    pub fn new() -> ChumBucket {
        return ChumBucket{best_move_p: None, best_move_a: None, best_rock_count: 0, best_range: 0};
    }

    //This AI is stupid and evaluates only based on:
    //Freedom of movement (Higher is better)
    //& Enemy rocks remaining. (Lower is better)
    pub fn eval_move(&mut self, game: &mut Game, ai_color: Tile) -> (&MovementAction, &MovementAction) {
        //First we get the opponent colour
        let opp_color = Self::get_opponent(ai_color);

        //Get all boards
        let home_b = *game.get_board(ai_color, Tile::Black).unwrap();
        let home_w = *game.get_board(ai_color, Tile::White).unwrap();
        let opp_b = *game.get_board(opp_color, Tile::Black).unwrap();
        let opp_w = *game.get_board(opp_color, Tile::White).unwrap();

        //Get all rock positions.
        let rock_pos_home_b = Self::get_rock_positions(&home_b, ai_color);
        let rock_pos_home_w = Self::get_rock_positions(&home_w, ai_color);
        let rock_pos_opp_b = Self::get_rock_positions(&opp_b, ai_color);
        let rock_pos_opp_w = Self::get_rock_positions(&opp_w, ai_color);

        /*
        for pos in rock_pos_home_b {
            Tile::get_possible_moves(&home_b, false, pos);
        } */

        unimplemented!();
    }

    pub fn get_rock_positions(b: &Board, target: Tile) -> Vec<(usize, usize)> {
        let board_state = b.get_state();
        let mut rock_positions: Vec<(usize, usize)> = Vec::new();
        //Go through each tile in the board and see if it's our rock coloures.
        for x in 0..=3 {
            for y in 0..=3 {
                if board_state[y][x] == target {
                    rock_positions.push((y, x));
                }
            }
        }
        return rock_positions;
    }

    fn get_opponent(color: Tile) -> Tile {
        match color {
            Tile::Black => return Tile::White,
            Tile::White => return Tile::Black,
            Tile::Empty => unimplemented!(),
        }
    }
}