use rand::{distributions::Alphanumeric, Rng};
use serde::Serialize;

use super::{game_board::Board, game_tile::Tile,};

#[derive(Debug, Clone, Serialize)]
pub struct Game {
    player_b: String,
    player_w: String,
    boards: [Board; 4],
    turn: Tile,
    winner: Tile, 
}

impl Game {
    pub fn new_game() -> Game {
        let board_bw = Board::new_board(Tile::Black, Tile::White);
        let board_bb = Board::new_board(Tile::Black, Tile::Black);

        let board_ww = Board::new_board(Tile::White, Tile::White);
        let board_wb = Board::new_board(Tile::White, Tile::Black);

        return Game {
            player_b: String::from("None"),
            player_w: String::from("None"),
            boards: [board_bw, board_bb, board_wb, board_ww],
            turn: Tile::Black,
            winner: Tile::Empty,
        };
    }

    pub fn next_turn(&mut self) {
        match self.turn {
            Tile::White => self.turn = Tile::Black,

            Tile::Black => self.turn = Tile::White,

            Tile::Empty => unimplemented!(),
        }
    }

    pub fn get_turn(&self) -> Tile{
        return self.turn;
    }

    pub fn get_players(&self) -> (String, String) {
        //Forgive me father for I have sinned.
        return (self.player_b.to_owned(), self.player_w.to_owned());
    }

    pub fn is_player(&self, s: &String) -> Tile {
        match s.as_str() {
            p if p == self.player_b => Tile::black(),
            p if p == self.player_w => Tile::white(),
            _ => Tile::empty(),
        }
    }

    pub fn set_winner(&mut self, t: &Tile){
        if self.winner == Tile::Empty {
            self.winner = t.to_owned();
        } 
    }

    pub fn has_winner(&self) -> bool{
        return self.winner != Tile::Empty;
    }

    pub fn add_player(&mut self, player_id: String) -> bool {
        // Check if player is not already assigned to player_b or player_w
        if self.player_b == "None" && self.player_w != player_id.clone() {
            self.player_b = player_id.clone();
            return true;
        } else if self.player_w == "None" && self.player_b != player_id.clone() {
            self.player_w = player_id.clone();
            return true;
        }
        return false;
    }

    pub fn get_board(&mut self, h: Tile, c: Tile) -> Option<&mut Board> {
        for board in &mut self.boards {
            if board.get_color() == c && board.get_home() == h {
                return Some(board);
            }
        }
        return None;
    }

    //Used for "fancy print" in CLI.
    pub fn display(&mut self) -> String {
        let mut disp: String = String::from("\n\n\tS H O B U\n\n");
        
        let red = "\x1b[31m";
        let green = "\x1b[32m";
        let reset = "\x1b[0m";

        let p_w_rope = format!("\n----------- {} ---------\n\n",  &self.player_w as &str);
        let p_b_rope = format!("\n----------- {} ---------\n\n",  &self.player_b as &str);

        //TRASH
        disp.push_str(&p_w_rope);
        for i in 0..4 as usize {
            for j in 0..4 as usize {
                disp.push_str(red);
                match self
                    .get_board(Tile::White, Tile::Black)
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
                    .get_board(Tile::White, Tile::White)
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
                    .get_board(Tile::Black, Tile::White)
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
                    .get_board(Tile::Black, Tile::Black)
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
        disp.push_str(&p_b_rope);
        
        return String::from(disp);
    }

    //Used for making a game lobby.
    pub fn generate_url() -> String {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(9)
            .map(char::from)
            .collect();
        return s;
    }
}
