use std::{cmp::Ord, f32};

use serde::{Deserialize, Serialize};

use super::game_board::Board;

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Tile {
    Empty,
    Black,
    White,
}

impl Tile {
    pub fn get_possible_moves(b: &Board, aggr: bool, cur_pos: (i8, i8)) -> Vec<(i8, i8)> {
        let boardstate = b.get_state();
        let mut movelist: Vec<(i8, i8)> = Vec::new();

        //If our tile is empty we have nada moves.
        if boardstate[cur_pos.0 as usize][cur_pos.1 as usize] == Tile::Empty{
            return movelist;
        }
        
        //[y, x]?, x → y ↓
        //Ok wtf did I do here?
        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0), (-1, -1), (-1, 1), (1, 1), (1, -1)];

        for (dy, dx) in directions.iter() {
            for i in 1..=2 as i8 {
                let new_pos = (cur_pos.0 + i * dy, cur_pos.1 + i * dx);

                if !Tile::is_valid(&b, cur_pos, new_pos, aggr) {
                    break;
                }
                movelist.push((new_pos.0, new_pos.1));
            }
        }
        return movelist;
    }

    pub fn is_valid(board: &Board, cur_pos: (i8, i8), new_pos: (i8, i8), aggr: bool) -> bool {

        //Boardstate
        let state = board.get_state();

        //Current position x and y.
        let start_y = cur_pos.0 as usize;
        let start_x = cur_pos.1 as usize;

        //New position x and y.
        let end_y = new_pos.0 as usize;
        let end_x = new_pos.1 as usize;

        //Movement deltas.
        let dy = new_pos.0 - cur_pos.0;
        let dx = new_pos.1 - cur_pos.1;

        //Movement directions.
        let dir_y = (dy as f32 / 2.0).round() as i8;
        let dir_x = (dx as f32 / 2.0).round() as i8;

        //i = size of step, either 1 or 2.
        let size = dy.abs().max(dx.abs());

        //step coordinates (used if move is 2 in size)
        let step_y = (cur_pos.0 + dir_y * 1) as usize;
        let step_x = (cur_pos.1 + dir_x * 1) as usize;

        //Check if outta range
        if end_x > 3 || end_y > 3 
        || step_y > 3 || step_x > 3 {
            return false;
        }

        //Passive move; invalid if anything is in its way.
        if !aggr {
            return state[end_y][end_x] == Tile::Empty 
            && state[step_y][step_x] == Tile::Empty;
        }

        //TODO: Rewrite this better.
        //Aggressive move.
        if aggr {
            //We may not push our own rocks.
            if state[end_y][end_x] == state[start_y][start_x] || state[step_y][step_x] == state[start_y][start_x] {
                return false;
            }

            //future rock positions if pushed:
            let rock_y = (cur_pos.0 + (size + 1) * dir_y) as usize;
            let rock_x = (cur_pos.1 + (size + 1) * dir_x) as usize;

            //In case the rock is pushed off the board.
            let on_board = !((rock_y) > 3 || (rock_x) > 3); 
            
            if on_board {
                //If a future rock position is not empty then the move is not valid.
                if size == 2 && state[rock_y][rock_x] != Tile::Empty {
                    if state[end_y][end_x] != Tile::Empty || state[step_y][step_x] != Tile::Empty {
                        return false;
                    }
                }
            }

            //Edge case, rogue rock squish.
            if size == 2 && state[end_y][end_x] != Tile::Empty 
            && state[step_y][step_x] != Tile::Empty && !on_board {
                return false;
            }

            //Check if a rock is behind our new position if we're pushing a rock. If it's empty we good.
            if size == 1 && state[end_y][end_x] != Tile::Empty && on_board {
                return state[rock_y][rock_x] == Tile::Empty;
            }

            //If the rock we're pushing is between us and the target tile.
            if size == 2 && state[step_y][step_x] != Tile::Empty {
                //Check if there is a rock where we're going.
                return state[end_y][end_x] == Tile::Empty;
            }
        }
        return true;
    }

    pub fn passive_move(b: &mut Board, cur_pos: (i8, i8), new_pos: (i8, i8)) -> bool {

        //Start pos
        let start_y = cur_pos.0 as usize;
        let start_x = cur_pos.1 as usize;

        //End pos
        let end_y = new_pos.0 as usize;
        let end_x = new_pos.1 as usize;
        

        //If the move is invalid, return false.
        if !Tile::is_valid(b, cur_pos, new_pos, false) {
            return false;
        }

        let mut boardstate = *b.get_state();
        
        let rock_me = boardstate[start_y][start_x];

        //Clear the old space
        boardstate[start_y][start_x] = Tile::Empty;

        //Move the rock
        boardstate[end_y][end_x] = rock_me;

        //Update state with new board.
        b.set_state(&boardstate);
        return true;
    }

    pub fn aggressive_move(b: &mut Board, cur_pos: (i8, i8), new_pos: (i8, i8)) -> bool {

        let cur_tile = b.to_owned().get_state()[cur_pos.0 as usize][cur_pos.1 as usize];
        if cur_tile == Tile::Empty {
            eprintln!("\nwtf are you doing? That's not a rock!\n"); 
        }

        //Movement detlas.
        let dx = new_pos.1 - cur_pos.1;
        let dy = new_pos.0 - cur_pos.0;

        //First and foremost check if the move is valid.
        if !Tile::is_valid(&b, cur_pos, new_pos, true) {
            return false;
        }

        let mut boardstate = *b.get_state();

        //Direction
        let dir_y = (dy as f32 / 2.0).round() as i8;
        let dir_x = (dx as f32 / 2.0).round() as i8;

        //Starting position of the aggressive rock.
        let start_y = cur_pos.0 as usize;
        let start_x = cur_pos.1 as usize;

        //The space between end and start (Only used if we move 2 steps)
        let step_y = (cur_pos.0 + 1 * dir_y) as usize;
        let step_x = (cur_pos.1 + 1 * dir_x) as usize;

        //Target position
        let end_y = new_pos.0 as usize;
        let end_x = new_pos.1 as usize;

        //End position for the rock we push.
        let rock_y = (new_pos.0 + 1 * dir_y) as usize;
        let rock_x = (new_pos.1 + 1 * dir_x) as usize;

        let mut stepping: bool = true;
        let mut on_board: bool = true;

        if (step_y, step_x) == (end_y, end_x) {
            stepping = false;
        }
        if rock_x > 3 || rock_y > 3{
            on_board = false;
        }

        //If the pushed rock is still on the board.
        if on_board {
            //If we move one step and a rock is there. Move the rock.
            if boardstate[end_y][end_x] != Tile::Empty && !stepping {
                boardstate[rock_y][rock_x] = boardstate[end_y][end_x];
            }
            //Leapfrog
            else if boardstate[step_y][step_x] != Tile::Empty && stepping {
                boardstate[rock_y][rock_x] = boardstate[step_y][step_x];
                boardstate[step_y][step_x] = Tile::Empty;
            }
            //Edge case; diagonal 2 step pushes that are still on the board.
            else if boardstate[end_y][end_x] != Tile::Empty && boardstate[step_y][step_x] == Tile::Empty && stepping {
                boardstate[rock_y][rock_x] = boardstate[end_y][end_x];
            }   
        }

        //If the rock gets pushed off the edge.
        if !on_board && stepping {
            boardstate[step_y][step_x] = Tile::Empty;
        }
        
        //Move the rock.
        boardstate[end_y][end_x] = boardstate[start_y][start_x];
        //Clear the previous Tile.
        boardstate[start_y][start_x] = Tile::Empty;
        //Uppdatera boardstate.
        b.set_state(&boardstate);

        return true;
    }
}
