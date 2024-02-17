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
    pub fn is_empty(t: Tile) -> bool {
        return t == Tile::Empty;
    }

    pub fn get_possible_moves(b: &Board, aggr: bool, cur_pos: (i8, i8)) -> Vec<(i8, i8)> {
        let boardstate = b.get_state();
        let mut movelist: Vec<(i8, i8)> = Vec::new();

        //If our tile is empty we have nada moves.
        if boardstate[cur_pos.0 as usize][cur_pos.1 as usize] == Tile::Empty{
            return movelist;
        }
        
        //[y, x]?, x → y ↓
        //Ok wtf did I do here?
        let directions = [
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
            (-1, -1),
            (-1, 1),
            (1, 1),
            (1, -1),
        ];

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

    //This is ugly but eh.
    pub fn is_valid(board: &Board, cur_pos: (i8, i8), new_pos: (i8, i8), aggr: bool) -> bool {

        let state = board.get_state();

        //Check if in range.
        let newy = new_pos.0 as usize;
        let newx = new_pos.1 as usize;

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

        //If outta range
        if newx > 3 || newy > 3 || step_y > 3 || step_x > 3 {
            return false;
        }

        //Passive move; invalid if anything is in its way.
        if !aggr {
            return state[newy][newx] == Tile::Empty && state[step_y][step_x] == Tile::Empty;
        }

        //Aggressive move.
        if aggr {
            //We may not push our own rocks.
            if state[newy][newx] == state[cur_pos.0 as usize][cur_pos.1 as usize] {
                return false;
            }

            //future rock positions if pushed:
            let rock_y = cur_pos.0 + (size + 1) * dir_y;
            let rock_x = cur_pos.1 + (size + 1) * dir_x;

            //In case the rock is pushed off the board.
            if (rock_y) > 3 || (rock_x) > 3 || (rock_y) < 0 || (rock_x) < 0 {
                return true;
            }

            //Check if a rock is behind our new position if we're pushing a rock. If it's empty we good.
            if size == 1 && state[newy][newx] != Tile::Empty {
                return state[rock_y as usize][rock_x as usize] == Tile::Empty;
            } else if size == 2 && state[step_y][step_x] != Tile::Empty {
                //If a future rock position is not empty then the move is not valid.
                if state[rock_y as usize][rock_x as usize] != Tile::Empty{
                    return false;
                }
                //Checka if there is a rock where we're going.
                return state[newy][newx] == Tile::Empty;
            }
        }
        return true;
    }

    pub fn passive_move(b: &mut Board, cur_pos: (i8, i8), new_pos: (i8, i8)) -> bool {

        //If thhe move is invalid, return false.
        if !Tile::is_valid(b, cur_pos, new_pos, false) {
            return false;
        }

        //If we move 0 steps the move is false.
        if cur_pos == new_pos {
            return false;
        }

        let mut boardstate = *b.get_state();
        
        let rock_me = boardstate[cur_pos.0 as usize][cur_pos.1 as usize];

        //Old space is empty
        boardstate[cur_pos.0 as usize][cur_pos.1 as usize] = Tile::Empty;

        //New space has the rock
        boardstate[new_pos.0 as usize][new_pos.1 as usize] = rock_me;

        //Update state with new board.
        b.set_state(&boardstate);
        return true;
    }

    pub fn aggressive_move(b: &mut Board, cur_pos: (i8, i8), new_pos: (i8, i8)) -> bool {

        let cur_tile = b.to_owned().get_state()[cur_pos.0 as usize][cur_pos.1 as usize];

        if Tile::is_empty(cur_tile) {
            eprintln!("\nwtf are you doing? That's not a rock!\n"); 
        }

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
        if on_board{
            //If we move one step and a rock is there. Move the rock.
            if boardstate[end_y][end_x] != Tile::Empty && !stepping{
                boardstate[rock_y][rock_x] = boardstate[end_y][end_x];
            }
            //Leapfrog
            else if boardstate[step_y][step_x] != Tile::Empty && stepping{
                boardstate[rock_y][rock_x] = boardstate[step_y][step_x];
                //Clear the spot behind us. (The D'Lcrantz method)
                boardstate[step_y][step_x] = Tile::Empty;
            }
            //Edge case; diagonal 2 step pushes that are still on the board.
            else if boardstate[end_y][end_x] != Tile::Empty && boardstate[step_y][step_x] == Tile::Empty && stepping{
                boardstate[rock_y][rock_x] = boardstate[end_y][end_x];
            }   
        }

        //If the rock gets pushed off the edge.
        if !on_board {
            if stepping {
                if boardstate[end_y][end_x] != Tile::Empty {
                    boardstate[end_y][end_x] = Tile::Empty;
                }
                else if boardstate[step_y][step_x] != Tile::Empty{
                    boardstate[step_y][step_x] = Tile::Empty;
                }
            }
        }

        /*
        [start][step][end][rock]- Case 1
        [start][step][end]- Case 2
        [start][end][rock]- Case 3
        [start][end]- Case 4
        */
        
        //Move the rock.
        boardstate[end_y][end_x] = boardstate[start_y][start_x];
        //Clear the previous Tile.
        boardstate[start_y][start_x] = Tile::Empty;
        //Uppdatera boardstate.
        b.set_state(&boardstate);

        return true;
    }
}
