use serde::{Deserialize, Serialize};

use super::game_tile::Tile;

#[derive(Clone, Copy, PartialEq, Debug, Serialize)]
pub struct Board {
    color: Color,
    home: Color,
    state: [[Tile; 4]; 4],
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Color {
    White,
    Black,
}

impl Board {
    //Creates a new board
    pub fn new_board(board_color: Color, home_color: Color) -> Board {
        return Board {
            color: board_color,
            home: home_color,
            state: Board::new_state(),
        };
    }

    //Create default board.
    fn new_state() -> [[Tile; 4]; 4] {
        //Creates a new empty boardstate.
        let mut board: [[Tile; 4]; 4] = [[Tile::empty(); 4]; 4];

        //Add rocks
        for i in 0..4 {
            //Top row (White)
            board[0][i] = Tile::white();
            //board[0][i] = Some(Stone::new(Color::White,  (0, i)));

            //Bottom row (Black)
            board[3][i] = Tile::black();
            //board[3][i] = Some(Stone::new(Color::Black,  (3, i)));
        }

        /*
        Detta returnar:
        [W][W][W][W]
        [ ][ ][ ][ ]
        [ ][ ][ ][ ]
        [B][B][B][B]

        Boarden displayas
        flipped för white.
         */
        return board;
    }

    pub fn get_state(&self) -> &[[Tile; 4]; 4] {
        return &self.state;
    }

    pub fn set_state(&mut self, new_state: &[[Tile; 4]; 4]) {
        self.state = *new_state;
    }

    pub fn check_winner(b: &Board) -> Option<Color> {
        let state = b.get_state();

        let has_white = state.iter().any(|row| row.contains(&Tile::White));
        let has_black = state.iter().any(|row| row.contains(&Tile::Black));

        match (has_white, has_black) {
            (true, true) => None,
            (false, true) => Some(Color::Black),
            (true, false) => Some(Color::White),
            _ => None,
        }
    }

    pub fn get_color(&self) -> Color {
        return self.color;
    }

    pub fn get_home(&self) -> Color {
        return self.home;
    }
}
