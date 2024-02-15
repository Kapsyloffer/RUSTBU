use serde::Serialize;

use super::game_tile::Tile;

#[derive(Clone, Copy, PartialEq, Debug, Serialize)]
pub struct Board {
    color: Tile,
    home: Tile,
    state: [[Tile; 4]; 4],
}

impl Board {
    //Creates a new board
    pub fn new_board(board_color: Tile, home_color: Tile) -> Board {
        return Board {
            color: board_color,
            home: home_color,
            state: Board::new_state(),
        };
    }

    //Create default board.
    fn new_state() -> [[Tile; 4]; 4] {
        //Creates a new empty boardstate.
        let mut board: [[Tile; 4]; 4] = [[Tile::Empty; 4]; 4];

        //Add rocks
        for i in 0..4 {
            //Top row (White)
            board[0][i] = Tile::White;

            //Bottom row (Black)
            board[3][i] = Tile::Black;
        }

        /*
        Detta returnar:
        [W][W][W][W]
        [ ][ ][ ][ ]
        [ ][ ][ ][ ]
        [B][B][B][B]
         */
        return board;
    }

    pub fn get_state(&self) -> &[[Tile; 4]; 4] {
        return &self.state;
    }

    pub fn set_state(&mut self, new_state: &[[Tile; 4]; 4]) {
        self.state = *new_state;
    }

    pub fn check_winner(b: &Board) -> Tile {
        let state = b.get_state();

        let has_white = state.iter().any(|row| row.contains(&Tile::White));
        let has_black = state.iter().any(|row| row.contains(&Tile::Black));

        match (has_white, has_black) {
            (false, true) => Tile::Black,
            (true, false) => Tile::White,
            _ => Tile::Empty,
        }
    }

    pub fn get_color(&self) -> Tile {
        return self.color;
    }

    pub fn get_home(&self) -> Tile {
        return self.home;
    }

    pub fn fancy_print(&self) -> String {
        let mut disp: String = String::new(); 
        for i in 0..4 as usize {
            for j in 0..4 as usize {
                match self.get_state()[i][j]
                {
                    Tile::White => disp.push_str("[W]"),
                    Tile::Black => disp.push_str("[B]"),
                    Tile::Empty => disp.push_str("[ ]"),
                }
            }
            disp.push_str("\n");
        }
        return disp;
    }
}
