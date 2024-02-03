use std::{cmp::Ord, f32};

use serde::Serialize;

use super::game_board::Board;

#[derive(Clone, Copy, PartialEq, Debug, Serialize)]
pub enum Tile {
    Empty,
    Black,
    White,
}

impl Tile {
    pub fn is_empty(t: Tile) -> bool {
        return t == Tile::Empty;
    }

    pub fn empty() -> Tile {
        return Tile::Empty;
    }

    pub fn white() -> Tile {
        return Tile::White;
    }

    pub fn black() -> Tile {
        return Tile::Black;
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

        let stepy = (cur_pos.0 + dy * 1) as usize;
        let stepx = (cur_pos.1 + dx * 1) as usize;

        //If outta range
        if newx > 3 || newy > 3 || stepy > 3 || stepx > 3 {
            return false;
        }

        //Passive
        if !aggr {
            if state[newy][newx] != Tile::Empty {
                return false;
            }
        }

        if aggr{
            //Knuffa ej våra egna stenar.
            if state[newy][newx] == state[cur_pos.0 as usize][cur_pos.1 as usize] {
                return false;
            }

            //future rock positions:
            let rocky = cur_pos.0 + (*size + 1) * dy;
            let rockx = cur_pos.0 + (*size + 1) * dx;

            //In case stenen vi puttar faller av boarden.
            if (rocky) > 3 || (rockx) > 3 || (rocky) < 0 || (rockx) < 0 {
                return true;
            }

            if *size == 1 && state[newy][newx] != Tile::Empty {
                //Checka om det finns en sten bakom stenen vi puttar. Om Tomt we good.
                return state[rocky as usize][rockx as usize] == Tile::Empty;
            } else if *size == 2 && state[stepy][stepx] != Tile::Empty {
                //If a future rock position is not empty then the move is not valid.
                if state[rocky as usize][rockx as usize] != Tile::Empty{
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
        /*
        Detta kommer ge typ:
        [W][B][ ][B]
        [ ][W][ ][ ]
        [ ][ ][ ][ ]
        [ ][w][B][W]

        Så vi tar cur_pos och flyttar till new_pos
        */
        let rock_me = boardstate[cur_pos.0 as usize][cur_pos.1 as usize];

        //Old space is empty
        boardstate[cur_pos.0 as usize][cur_pos.1 as usize] = Tile::empty();

        //New space has the rock
        boardstate[new_pos.0 as usize][new_pos.1 as usize] = rock_me;

        //Update state with new board.
        b.set_state(&boardstate);
        return true;
    }

    pub fn aggressive_move(b: &mut Board, cur_pos: (i8, i8), new_pos: (i8, i8)) -> bool {
        let cur_tile = b.to_owned().get_state()[cur_pos.0 as usize][cur_pos.1 as usize];
        if Tile::is_empty(cur_tile) {
            eprintln!("\nwtf are you doing. That's not a rock!\n"); 
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

        /*
        b.get_state() kommer ge typ:
        [ ][ ][ ][ ]      [ ][ ][ ][ ]      [ ][ ][ ][ ]
        [ ][ ][ ][B]      [ ][ ][ ][W]      [ ][ ][ ][W]
        [ ][ ][ ][ ]  =>  [ ][ ][ ][ ]  =>  [ ][ ][ ][ ]
        [ ][w][ ][ ]      [ ][ ][ ][ ]      [ ][ ][ ][ ]

        [ ][ ][ ][ ]      [ ][ ][ ][ ]      [ ][B][ ][ ]
        [ ][ ][ ][ ]      [ ][W][ ][ ]      [ ][W][ ][ ]
        [ ][B][ ][ ]  =>  [ ][B][ ][ ]  =>  [ ][ ][ ][ ]
        [ ][w][ ][ ]      [ ][ ][ ][ ]      [ ][ ][ ][ ]

        [ ][W][B][ ]      [ ][ ][B][W]      [ ][ ][ ][W]
        [ ][ ][ ][ ]      [ ][ ][ ][ ]      [ ][ ][ ][ ]
        [ ][ ][ ][ ]  =>  [ ][ ][ ][ ]  =>  [ ][ ][ ][ ]
        [ ][ ][ ][ ]      [ ][ ][ ][ ]      [ ][ ][ ][ ]

        Hopefully
        */
        let rockx = (new_pos.1 + 1 * dir.1) as usize;
        let rocky = (new_pos.0 + 1 * dir.0) as usize;

        let stepy = new_pos.0 - 1 * dir.0;
        let stepx = new_pos.1 - 1 * dir.1;

        //TODO: Get rid of this nightmare code.

        //If rock is still on board.
        if rockx <= 3 && rocky <= 3{
            //Denna kallas in case vi landar på stenen.
            if boardstate[new_pos.0 as usize][new_pos.1 as usize] != Tile::Empty && size == 1{
                //Flytta stenen till nextpos.
                boardstate[rocky as usize][rockx as usize] = boardstate[new_pos.0 as usize][new_pos.1 as usize];
                //Set bår sten till nya positionen.
                boardstate[new_pos.0 as usize][new_pos.1 as usize] = boardstate[cur_pos.0 as usize][cur_pos.1 as usize];
            }
            //Denna kallas in case vi hoppar över stenen.
            if boardstate[stepy as usize][stepx as usize] != Tile::Empty && size == 2{
                //Sätt nästa position till stenens position.
                boardstate[rocky as usize][rockx as usize] = boardstate[stepy as usize][stepx as usize];
                //Rensa platsen 1 steg bakom oss. (D'Lcrantz metoden)
                boardstate[stepy as usize][stepx as usize] = Tile::empty();
            }
        //OM STENEN FALLER AV.
        }else if rockx > 3 || rocky > 3{
            //Sätt nya posen till vår färg
            boardstate[new_pos.0 as usize][new_pos.1 as usize] = boardstate[cur_pos.0 as usize][cur_pos.1 as usize];
            //Rensa platsen 1 steg bakom oss. (D'Lcrantz metoden)
            boardstate[stepy as usize][stepx as usize] = Tile::empty();
            //redundant? Eh.
            boardstate[cur_pos.0 as usize][cur_pos.1 as usize] = Tile::empty();
        }
        
        //Rensa förra platsen.
        boardstate[cur_pos.0 as usize][cur_pos.1 as usize] = Tile::empty();
        //Uppdatera boardstate.
        b.set_state(&boardstate);

        return true;
    }
}
