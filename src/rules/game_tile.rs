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

        //Om vår tile är tom har vi nada moves.
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

                if Tile::is_valid(boardstate, cur_pos, new_pos, &i, aggr, (&dy, &dx)) {
                    movelist.push((new_pos.0, new_pos.1)); //this is so crummy.
                    continue;
                }
                break;
            }
        }
        return movelist;
    }

    pub fn is_valid(state: &[[Tile; 4]; 4], cur_pos: (i8, i8), new_pos: (i8, i8), size: &i8, aggr: bool, (dy, dx): (&i8, &i8)) -> bool {
        //Check if in range.
        let newy = new_pos.0 as usize;
        let newx = new_pos.1 as usize;

        let step_y = (cur_pos.0 + dy * 1) as usize;
        let step_x = (cur_pos.1 + dx * 1) as usize;

        //If outta range
        if newx > 3 || newy > 3 || step_y > 3 || step_x > 3 {
            return false;
        }

        //Passive
        if !aggr {
            return state[newy][newx] == Tile::Empty;
        }

        if aggr{
            //Knuffa ej våra egna stenar.
            if state[newy][newx] == state[cur_pos.0 as usize][cur_pos.1 as usize] {
                return false;
            }

            //future rock positions:
            let rock_y = cur_pos.0 + (*size + 1) * dy;
            let rock_x = cur_pos.1 + (*size + 1) * dx;

            //In case stenen vi puttar faller av boarden.
            if (rock_y) > 3 || (rock_x) > 3 || (rock_y) < 0 || (rock_x) < 0 {
                return true;
            }

            if *size == 1 && state[newy][newx] != Tile::Empty {
                //Checka om det finns en sten bakom stenen vi puttar. Om Tomt we good.
                return state[rock_y as usize][rock_x as usize] == Tile::Empty;
            } else if *size == 2 && state[step_y][step_x] != Tile::Empty {
                //If a future rock position is not empty then the move is not valid.
                if state[rock_y as usize][rock_x as usize] != Tile::Empty{
                    return false;
                }
                //Checka om det finns en sten bakom stenen vi puttar.
                return state[newy][newx] == Tile::Empty;
            }
        }

        return true;
    }

    pub fn passive_move(b: &mut Board, cur_pos: (i8, i8), new_pos: (i8, i8)) -> bool {

        let dx = new_pos.1 - cur_pos.1;
        let dy = new_pos.0 - cur_pos.0;

        //i = antal steps, 1 eller 2
        let i = dy.abs().max(dx.abs());

        if !Tile::is_valid(b.get_state(), cur_pos, new_pos, &i, false, (&dy, &dx)) {
            return false;
        }

        if dx == 0 && dy == 0 {
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

        let dir = (
            (dy as f32 / 2.0).round() as i8,
            (dx as f32 / 2.0).round() as i8,
        );

        //i = antal steps, 1 eller 2
        let size = dy.abs().max(dx.abs());

        //First and foremost check if the move is valid.
        if !Tile::is_valid(b.get_state(), cur_pos, new_pos, &size, true, (&dir.0, &dir.1)) {
            return false;
        }

        let mut boardstate = *b.get_state();

        let dir = (
            (dy as f32 / 2.0).round() as i8,
            (dx as f32 / 2.0).round() as i8,
        );

        //Starting position of the aggressive rock.
        let start_y = cur_pos.0 as usize;
        let start_x = cur_pos.1 as usize;

        //The space between end and start (Only used if we move 2 steps)
        let step_y = (cur_pos.0 + 1 * dir.0) as usize;
        let step_x = (cur_pos.1 + 1 * dir.1) as usize;

        //Target position
        let end_y = new_pos.0 as usize;
        let end_x = new_pos.1 as usize;

        //End position for the rock we push.
        let rock_y = (new_pos.0 + 1 * dir.0) as usize;
        let rock_x = (new_pos.1 + 1 * dir.1) as usize;

        let mut stepping: bool = true;
        let mut on_board: bool = true;
        if step_x == end_x && step_y == end_y {
            stepping = false;
        }
        if rock_x > 3 || rock_y > 3{
            on_board = false;
        }

        //If the pushed rock is still on the board.
        if on_board{
            if boardstate[end_y][end_x] != Tile::Empty && !stepping{
                boardstate[rock_y][rock_x] = boardstate[end_y][end_x];
            }
            else if boardstate[step_y][step_x] != Tile::Empty && stepping{
                boardstate[rock_y][rock_x] = boardstate[step_y][step_x];
                //Rensa platsen 1 steg bakom oss. (D'Lcrantz metoden)
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
        
        //Flytta stenen.
        boardstate[end_y][end_x] = boardstate[start_y][start_x];
        //Rensa förra platsen.
        boardstate[start_y][start_x] = Tile::Empty;
        //Uppdatera boardstate.
        b.set_state(&boardstate);

        return true;
    }
}
