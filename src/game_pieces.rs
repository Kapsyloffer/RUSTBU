#![allow(dead_code)]
#![allow(unused)]

use std::
{
    f32, 
    primitive, 
    cmp::Ord
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Board
{
    color: Color,
    home: Color,
    state: [[Tile; 4]; 4]
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tile
{
    Empty,
    Black,
    White
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color 
{
    White,
    Black,
}

impl Board
{
    //Creates a new board
    pub fn new_board(board_color: Color, home_color: Color) -> Board
    {
        return Board
        {
            color: board_color,
            home: home_color,
            state: Board::new_state()
        }
    }

    //Create default board.
    fn new_state() -> [[Tile; 4]; 4]
    {
        //Creates a new empty boardstate.
        let mut board: [[Tile; 4]; 4] = [[Tile::empty(); 4]; 4];

        //Add rocks
        for i in 0..4 
        {
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

    pub fn get_state(&self) -> &[[Tile; 4]; 4]
    {
        return &self.state;
    }

    pub fn set_state (&mut self, new_state: [[Tile; 4]; 4])
    {
        self.state = new_state;
    }

    pub fn check_winner(b: &Board) -> Option<Color> 
    {
        let state = b.get_state();
    
        let has_white = state.iter().any(|row| row.contains(&Tile::White));
        let has_black = state.iter().any(|row| row.contains(&Tile::Black));
    
        match (has_white, has_black) 
        {
            (true, true) => None,
            (false, true) => Some(Color::Black),
            (true, false) => Some(Color::White),
            _ => None,
        }
    }

    pub fn get_color(&self) -> Color
    {
        return self.color;
    }
}

impl Tile
{
    pub fn empty() -> Tile
    {
        return Tile::Empty;
    }

    pub fn white() -> Tile
    {
        return Tile::White;
    }

    pub fn black() -> Tile
    {
        return Tile::Black;
    }

    pub fn passive_move(&self, b: &mut Board, cur_pos: (i8, i8), new_pos: (i8, i8)) -> (bool, (i8, i8), Color)
    {
        if !self.get_possible_moves(b, false, cur_pos).contains(&new_pos)
        {
            return (false, (0,0), b.get_color());
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

        b.set_state(boardstate);

        //Får ut storleken flyttad så vi kan slänga in den i aggr.
        let sizediff = ((cur_pos.0 - new_pos.0).abs(), (cur_pos.1 - new_pos.1).abs());
        return (true, sizediff, b.get_color());
    }

    pub fn get_possible_moves(&self, b: &Board, aggr: bool, cur_pos: (i8, i8)) -> Vec<(i8, i8)>
    {
        let mut boardstate = b.get_state();

        let mut movelist: Vec<(i8, i8)> = Vec::new();
        //0 = y, 1 = x
        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0), (-1, -1), (-1, 1), (1, 1), (1, -1)];
        //So a move to the left is [1, 0]

        for (dy, dx) in directions.iter()
        {
            for i in 1..3 as i8
            {
                let new_pos = (cur_pos.0 + i * dy, cur_pos.1 + i * dx);

                if self.is_valid(boardstate, cur_pos, new_pos, &i, aggr, (&dy, &dx))
                {
                    println!("ADDED {} {}, DIRECTION: {} {}, DIFF: {} {}", new_pos.0, new_pos.1, dy, dx, (cur_pos.0 - new_pos.0).abs(), (cur_pos.1 - new_pos.1).abs());
                    movelist.push((new_pos.0, new_pos.1)); //this is so crummy.
                    continue;
                }
                break;
            }
        }
        return movelist;
    }

    pub fn is_valid(&self, state: &[[Tile; 4]; 4], cur_pos: (i8, i8), new_pos: (i8, i8), i: &i8, aggr: bool, (dy, dx): (&i8, &i8)) -> bool
    {
        //Check if in range.
        let newy = new_pos.0 as usize;
        let newx = new_pos.1 as usize;

        let stepy = (cur_pos.0 + 1 * dy) as usize;
        let stepx = (cur_pos.1  + 1 * dx) as usize;

        //If outta range
        if newx > 3 || newy > 3
        {
            return false;
        }

        //Passive
        if !aggr 
        {
            if state[newy][newx] != Tile::Empty 
            {
                return false;
            }
        } 
        
        if aggr //Det här är gigacrummy. TODO: Make less crummy.
        {
            //Knuffa ej våra egna stenar.
            if state[newy][newx] == state[cur_pos.0 as usize][cur_pos.1 as usize]
            {
                return false;
            }

            //Om rutan bakom är tom, och rutan har en sten. Move size 1
            if * i == 1 && state[newy][newx] != Tile::Empty && state[(cur_pos.0 + 2 * dy) as usize][(cur_pos.1 + 2 * dx) as usize] == Tile::Empty
            {
                return true;
            }
            
            //Om rutan bakom har en sten, och rutan har en sten. Move size 1
            if * i == 1 && state[newy][newx] != Tile::Empty && state[(cur_pos.0 + 2 * dy) as usize][(cur_pos.1 + 2 * dx) as usize] != Tile::Empty
            {
                return false;
            }

            //Om rutan bakom har ing en sten, och rutan har ingen sten. Move size 2
            if state[stepy][stepx] != Tile::Empty && *i == 2 && state[newy][newx] != Tile::Empty
            {
                return false;
            }
            
            //Om rutan bakom har ingen sten, och rutan har en sten. Move size 2
            if state[stepy][stepx] != Tile::Empty && *i == 2 && state[newy][newx] == Tile::Empty
            {
                return true;
            }
        }
        return true;
    }

    pub fn aggressive_move(&self, b: &mut Board, cur_pos: (i8, i8), diff: (i8, i8), prev_colour: Color) -> bool
    {
        let new_pos = (cur_pos.0 + diff.0, cur_pos.1 + diff.1);
        //Om tidigare movet gjordes på en board av samma färg.
        if b.get_color() == prev_colour
        {
            return false;
        }

        //Om draget inte finns
        if !self.get_possible_moves(b, true, cur_pos).contains(&new_pos)
        {
            return false;
        }
        
        let mut boardstate = *b.get_state();
        
        /*let rock_me = boardstate[cur_pos.0 as usize][cur_pos.1 as usize];
        let rock_notme = boardstate[new_pos.0 as usize][new_pos.1 as usize];

        //Old space is empty
        boardstate[cur_pos.0 as usize][cur_pos.1 as usize] = Tile::empty();

        //New space has the rock
        boardstate[new_pos.0 as usize][new_pos.1 as usize] = rock_me;*/

        //Move previously occupying rock
        //Get direction:
        /*
            0 / 2 = 0,
            1 / 2 ceil = 1
            -2 / 2 ceil = -1
            med dir kan vi stega x antal steg.
         */
        let dir = ((diff.0 as f32 / 2.0).ceil() as i8, (diff.1 as f32 / 2.0).ceil() as i8);
        let size = diff.0.abs().max(diff.1.abs());
       
        /*
        Detta kommer ge typ:
        [ ][ ][ ][ ]      [ ][ ][ ][ ]      [ ][ ][ ][ ]
        [ ][ ][ ][B]      [ ][ ][ ][W]      [ ][ ][ ][W]
        [ ][ ][ ][ ]  =>  [ ][ ][ ][ ]  =>  [ ][ ][ ][ ]
        [ ][w][ ][ ]      [ ][ ][ ][ ]      [ ][ ][ ][ ]

        [ ][ ][ ][ ]      [ ][ ][ ][ ]      [ ][B][ ][ ]
        [ ][ ][ ][ ]      [ ][W][ ][ ]      [ ][W][ ][ ]
        [ ][B][ ][ ]  =>  [ ][B][ ][ ]  =>  [ ][ ][ ][ ]
        [ ][w][ ][ ]      [ ][ ][ ][ ]      [ ][ ][ ][ ]

        Hopefully
        */

        let still_on_board = self.is_valid(b.get_state(), new_pos, (new_pos.0 + 1 * dir.0, new_pos.1 + 1 * dir.1), &size, true, (&dir.0, &dir.1));
        let push_pos: (i8, i8) = ((new_pos.0 + 1 * dir.0), (new_pos.1 + 1 * dir.1));

        if size > 1
        {
            if still_on_board
            {
                let rocky = boardstate[(new_pos.0 +1 * dir.0) as usize][(new_pos.1 +1 * dir.1) as usize];
                boardstate[push_pos.0 as usize][push_pos.1 as usize] = rocky;
            }

            //Sätt nya posen till vår färg
            boardstate[new_pos.0 as usize][new_pos.1 as usize] = boardstate[cur_pos.0 as usize][cur_pos.1 as usize];

            //Rensa förra.
            boardstate[(new_pos.0 -1 * dir.0) as usize][(new_pos.1 -1 * dir.1) as usize] = Tile::empty();
        }
        else 
        {
            if still_on_board
            {
                
            }

            boardstate[new_pos.0 as usize][new_pos.1 as usize] = boardstate[cur_pos.0 as usize][cur_pos.1 as usize];
            boardstate[cur_pos.0 as usize][cur_pos.1 as usize] = Tile::empty();
        }

        /* 
        //Om nya platsen inte är tom, checka om stenen hamnar out of bounds.
        if boardstate[new_pos.0 as usize][new_pos.1 as usize] != Tile::Empty
        {
            //Om den inte gör det flyttar vi upp den ett steg.
            if self.is_valid(b.get_state(), new_pos, (new_pos.0 + 1 * dir.0, new_pos.1 + 1 * dir.1), &size, true, (&dir.0, &dir.1))
            {
                boardstate[(new_pos.0 + 1 * dir.0) as usize][(new_pos.1 + 1 * dir.1) as usize] = boardstate[new_pos.0 as usize][new_pos.1 as usize];
            }
            boardstate[new_pos.0 as usize][new_pos.1 as usize] = *self;
        }
        //Om den är tom, chcka innan och se om den har en sten.
        else if boardstate[(new_pos.0 -1 * dir.0) as usize][(new_pos.1 -1 * dir.1) as usize] != Tile::Empty
            {
                if self.is_valid(b.get_state(), new_pos, (new_pos.0 + 1 * dir.0, new_pos.1 + 1 * dir.1), &size, true, (&dir.0, &dir.1))
                {
                    boardstate[(new_pos.0 + 1 * dir.0) as usize][(new_pos.1 + 1 * dir.1) as usize] = boardstate[(new_pos.0 -1 * dir.0) as usize][(new_pos.1 -1 * dir.1) as usize];
                }
                boardstate[new_pos.0 as usize][new_pos.1 as usize] = *self;
            }
        else //Annars om inga stenar, skjut bara ena stenen.
        {
            boardstate[new_pos.0 as usize][new_pos.1 as usize] = *self;
        }*/


        b.set_state(boardstate);

        return true;
    }
}