use crate::game_pieces::*;

#[cfg(test)]

#[test]
fn state_init()
{
    let board = Board::new_board(Color::Black, Color::Black);
    let stone = Stone::new(Color::White, (0,0));
    
    let top_left_stone = board.get_state()[0][0].as_ref().unwrap();
    let bot_right_stone = board.get_state()[3][3].as_ref().unwrap();

    //White
    assert_eq!(top_left_stone.get_color(), stone.get_color());
    //Black
    assert_ne!(bot_right_stone.get_color(), stone.get_color());
}

#[test]
fn movement()
{
    assert_eq!(1, 1);
}