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
            board[3][i] = Some(Stone::new(Color::Black,  (3, i)));
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

    fn get_pos(&self) -> (usize, usize)
    {
        return self.position;
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
            for i in 1..3 as i8
            {
                let new_pos = (cur_pos.0 + i * dy, cur_pos.1 + i * dx);

                if self.is_valid(boardstate, cur_pos, new_pos, &i, aggr, (&dy, &dx))
                {
                    println!("ADDED {} {}, DIRECTION: {} {}", new_pos.0, new_pos.1, dy, dx);
                    movelist.push((new_pos.0 as usize, new_pos.1 as usize)); //this is so crummy.
                    continue;
                }
                break;
            }
        }
        return movelist;
    }

    pub fn is_valid(&self, state: &Vec<Vec<Option<Stone>>>, cur_pos: (i8, i8), new_pos: (i8, i8), i: &i8, aggr: bool, (dy, dx): (&i8, &i8)) -> bool
    {
        //Check if in range.
        let newy = new_pos.0 as usize;
        let newx = new_pos.1 as usize;

        let stepy = (cur_pos.0 + 1 * dy) as usize;
        let stepx = (cur_pos.1  + 1 * dx) as usize;

        //If outta range
        if newx < 0 || newx > 3 || newy < 0 || newy > 3
        {
            return false;
        }

        //Passive
        if !aggr 
        {
            if state[newy][newx].is_some() 
            {
                return false;
            }
        } 
        
        if aggr //Det här är gigacrummy. TODO: Make less crummy.
        {
            //Knuffa ej våra egna stenar.
            if let Some(rock) = state[newy][newx] 
            {
                if rock.get_color() == self.get_color() 
                {
                    return false; 
                }
            }

            //Om rutan bakom är tom, och rutan har en sten. Move size 1
            if * i == 1 && state[newy][newx].is_some() && state[(cur_pos.0 + 2 * dy) as usize][(cur_pos.1 + 2 * dx) as usize].is_none()
            {
                return true;
            }
            
            //Om rutan bakom har en sten, och rutan har en sten. Move size 1
            if * i == 1 && state[newy][newx].is_some() && state[(cur_pos.0 + 2 * dy) as usize][(cur_pos.1 + 2 * dx) as usize].is_some()
            {
                return false;
            }

            //Om rutan bakom har ing en sten, och rutan har ingen sten. Move size 2
            if state[stepy][stepx].is_some() && *i == 2 && state[newy][newx].is_some()
            {
                return false;
            }
            
            //Om rutan bakom har ingen sten, och rutan har en sten. Move size 2
            if state[stepy][stepx].is_some() && *i == 2 && state[newy][newx].is_none()
            {
                return true;
            }
        }
        return true;
    }

    pub fn aggressive_move() -> ()
    {
        //todo: Somehow kolla vad färgen på tidigare boarden var.
        //todo: move buffer för passiv och aggressiv?
    }
}