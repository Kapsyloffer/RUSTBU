use rand::{distributions::Alphanumeric, Rng};
use super::game_board::{Board, Color};



//#[derive(Serialize, Deserialize, Debug)]
#[derive(Debug)]
pub struct Game
{
    player_b: Option<String>,
    player_w: Option<String>,
    boards: [Board; 4],
    turn: Color,
}

impl Game
{
    pub fn new_game () -> Game
    {
        let board_bw = Board::new_board(Color::Black, Color::White);
        let board_bb = Board::new_board(Color::Black, Color::Black);

        let board_ww = Board::new_board(Color::White, Color::White);
        let board_wb = Board::new_board(Color::White, Color::Black);

        return Game{player_b: None, player_w: None, boards: [board_bw, board_bb, board_wb, board_ww], turn: Color::Black};
    }

    pub fn next_turn(&mut self)
    {
        match self.turn
        {
            Color::White => self.turn = Color::Black,
            
            Color::Black => self.turn = Color::White,
        }
    }

    pub fn get_players(&self) -> (Option<String>, Option<String>)
    {
        //Forgive me father for I have sinned.
        return (self.player_b.to_owned(), self.player_w.to_owned());
    }

    pub fn get_boards(&self) -> [Board; 4]
    {
        return self.boards;
    }

    pub fn get_board(&self, h: Color, c: Color) -> Option<Board>
    {
        for board in *&self.boards 
        {
           if board.get_color() == c && board.get_home() == h
           {
                return Some(board);
           }
           else 
           {
                continue;   
           }
        }

        return None
    }

    pub fn generate_url() -> String
    {
        let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(9)
        .map(char::from)
        .collect();
        return s;
    }
}