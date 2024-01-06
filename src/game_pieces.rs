#[allow(unused)]
#[allow(dead_code)]
pub (crate) struct Board
{
    color: Color,
    home: Color,
    state: Vec<Vec<Option<Stone>>>
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub (crate) struct Stone
{
    color: Color,
    position: (usize, usize) 
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
    fn new_state() -> Vec<Vec<Option<Stone>>>
    {
        //Creates a new empty boardstate.
        let mut board: Vec<Vec<Option<Stone>>> = 
        vec![
            vec![None, None, None, None],
            vec![None, None, None, None],
            vec![None, None, None, None],
            vec![None, None, None, None],
        ];

        //Add rocks
        for i in 0..4 
        {
            //Top row (White)
            board[0][i] = Some(Stone::new(Color::White,  (0, i)));

            //Bottom row (Black)
            board[3][i] = Some(Stone::new(Color::Black,  (0, i)));
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

    pub fn get_state(&self) -> &Vec<Vec<Option<Stone>>>
    {
        return &self.state;
    }

    pub fn set_state(&mut self, new_state: Vec<Vec<Option<Stone>>>)
    {
        self.state = new_state;
    }
}

impl Stone
{
    pub (crate) fn new(c: Color, pos: (usize, usize)) -> Stone
    {
        return Stone
        {
            color: c,
            position: pos
        }
    }
    
    pub fn get_color(&self) -> Color
    {
        return self.color;
    }

    pub fn passive_move() -> ()
    {
        /*
        Detta kommer ge typ:
        [W][B][ ][B]
        [ ][W][ ][ ]
        [ ][ ][ ][ ]
        [ ][w][B][W]

        Om jag väljer lilla w ska jag ha movement i 
        1x vänster, 1x vänster upp, 1x upp, 2x upphöger, 0x resten.
        
        Ta possible moves, 
        om jag tar en som flyttar 2
        så får jag size 2 och direction som skickas till aggressive.
        */
    }

    pub fn get_possible_moves(&self, b: &Board, aggr: bool) -> Vec<(usize, usize)>
    {
        let mut boardstate = b.get_state();
        let cur_pos = (self.position.0 as i8, self.position.1 as i8); //0 = y, 1 = x

        let mut movelist: Vec<(usize, usize)> = Vec::new();
        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0), (-1, -1), (-1, 1), (1, 1), (1, -1)];

        //todo: a move to the left is a [1, 0] 
        //that gets added to the rock pos if movement.

        for (dy, dx) in directions.iter()
        {
            for i in 0..3 as i8
            {
                let newpos = (cur_pos.0 + i * dy, cur_pos.1 + i * dx);

                //free reign.
                if self.is_valid(boardstate, newpos, aggr, &i, (&dy, &dx))
                {
                    movelist.push((newpos.0 as usize, newpos.1 as usize)); //this is so crummy.
                    continue;
                }
                //om en sten står i vägen.
                else if i == 1 && !aggr{
                    break;
                }
            }
        }

        return movelist;
    }

    pub fn is_valid(&self, state: &Vec<Vec<Option<Stone>>>, pos: (i8, i8), aggr: bool, i: &i8, (dy, dx): (&i8, &i8)) -> bool
    {
        //Check if in range.
        if pos.1 < 0 || pos.1 > 3 || pos.0 < 0 || pos.0 > 3
        {
            return false;
        }

        //Gör alla passive only checks här-
        if !aggr
        {
            if state[pos.0 as usize][pos.1 as usize] != None
            {
                return false;
            }
        }

        //Om stället vi checkar inte är tomt, vi kollar movement size 1, och det är aggressivt.
        /*
        t.ex.
        [ ][ ][ ][ ]
        [ ][W][B][ ] Good
        [ ][ ][ ][ ]
        [ ][ ][ ][ ]

        
        [ ][ ][ ][ ]
        [ ][W][B][B] Bad
        [ ][ ][ ][ ]
        [ ][ ][ ][ ]

        [ ][ ][ ][ ]
        [ ][W][W][ ] VERY Bad 
        [ ][ ][ ][ ]
        [ ][ ][ ][ ]

        Om vi åker mot höger.
         */
        if state[pos.0 as usize][pos.1 as usize] != None  && *i == 1
        {

            //Du får ej knuffa dina egna stenar.
            if state[pos.0 as usize][pos.1 as usize].unwrap().get_color() == self.get_color() 
            {
                return false;
            }

            if state[(pos.0 + 1 * dy) as usize][(pos.1 + 1 * dx) as usize] == None
            {
                return true;
            }
            else 
            {
                return false;
            }
        }

        //Om det står en sten framför en men bakanför är tom, och vi är aggressiva.
        

        return true;
    }

    pub fn aggressive_move() -> ()
    {
        //todo: Somehow kolla vad färgen på tidigare boarden var.
        //todo: move buffer för passiv och aggressiv?
    }
}