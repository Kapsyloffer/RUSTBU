use rand::{distributions::Alphanumeric, Rng};
use serde::Serialize;

use super::{
    game_board::{Board, Color},
    game_tile::Tile,
};

//#[derive(Serialize, Deserialize, Debug)]
#[derive(Debug, Clone, Serialize)]
pub struct Game {
    player_b: Option<String>,
    player_w: Option<String>,
    boards: [Board; 4],
    turn: Color,
}

impl Game {
    pub fn new_game() -> Game {
        let board_bw = Board::new_board(Color::Black, Color::White);
        let board_bb = Board::new_board(Color::Black, Color::Black);

        let board_ww = Board::new_board(Color::White, Color::White);
        let board_wb = Board::new_board(Color::White, Color::Black);

        return Game {
            player_b: None,
            player_w: None,
            boards: [board_bw, board_bb, board_wb, board_ww],
            turn: Color::Black,
        };
    }

    pub fn next_turn(&mut self) {
        match self.turn {
            Color::White => self.turn = Color::Black,

            Color::Black => self.turn = Color::White,
        }
    }

    pub fn get_players(&self) -> (Option<String>, Option<String>) {
        //Forgive me father for I have sinned.
        return (self.player_b.to_owned(), self.player_w.to_owned());
    }

    pub fn get_boards(&self) -> [Board; 4] {
        return self.boards;
    }

    pub fn set_player(&mut self, _c: Option<Color>) {
        todo!()
    }

    pub fn get_board(&mut self, h: Color, c: Color) -> Option<&mut Board> {
        for board in &mut self.boards {
            if board.get_color() == c && board.get_home() == h {
                return Some(board);
            }
        }
        return None;
    }

    pub fn display(&mut self) -> String {
        let mut disp: String = String::from("\n\n\tS H O B U\n\n");
        let red = "\x1b[31m";
        let green = "\x1b[32m";
        let reset = "\x1b[0m";

        //TRASH
        disp.push_str("\n----------- WHITE ---------\n\n");
        for i in 0..4 as usize {
            for j in 0..4 as usize {
                disp.push_str(red);
                match self
                    .get_board(Color::White, Color::Black)
                    .unwrap()
                    .get_state()[i][j]
                {
                    Tile::White => disp.push_str("[W]"),
                    Tile::Black => disp.push_str("[B]"),
                    Tile::Empty => disp.push_str("[ ]"),
                }
                disp.push_str(reset);
            }
            disp.push_str("   ");
            for j in 0..4 as usize {
                disp.push_str(green);
                match self
                    .get_board(Color::White, Color::White)
                    .unwrap()
                    .get_state()[i][j]
                {
                    Tile::White => disp.push_str("[W]"),
                    Tile::Black => disp.push_str("[B]"),
                    Tile::Empty => disp.push_str("[ ]"),
                }
                disp.push_str(reset);
            }
            disp.push_str("\n");
        }
        disp.push_str("\n---------------------------\n\n");
        for i in 0..4 as usize {
            for j in 0..4 as usize {
                disp.push_str(green);
                match self
                    .get_board(Color::Black, Color::White)
                    .unwrap()
                    .get_state()[i][j]
                {
                    Tile::White => disp.push_str("[W]"),
                    Tile::Black => disp.push_str("[B]"),
                    Tile::Empty => disp.push_str("[ ]"),
                }
                disp.push_str(reset);
            }
            disp.push_str("   ");
            for j in 0..4 as usize {
                disp.push_str(red);
                match self
                    .get_board(Color::Black, Color::Black)
                    .unwrap()
                    .get_state()[i][j]
                {
                    Tile::White => disp.push_str("[W]"),
                    Tile::Black => disp.push_str("[B]"),
                    Tile::Empty => disp.push_str("[ ]"),
                }
                disp.push_str(reset);
            }
            disp.push_str("\n");
        }
        disp.push_str("\n----------- BLACK---------\n\n");
        return String::from(disp);
    }

    pub fn generate_url() -> String {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(9)
            .map(char::from)
            .collect();
        return s;
    }
}
