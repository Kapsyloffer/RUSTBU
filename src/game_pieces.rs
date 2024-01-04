#[allow(unused)]
pub (crate) struct Board
{
    color: Color,
    home: Color,
    state: Vec<Vec<Option<Stone>>>
}

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

        return board;
    }

    pub fn get_state(&self) -> &Vec<Vec<Option<Stone>>>
    {
        return &self.state;
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
}