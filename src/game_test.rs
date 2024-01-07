use crate::game_pieces::*;

#[cfg(test)]

#[test]
fn state_init()
{
    let board = Board::new_board(Color::Black, Color::Black);
    
    let top_left_stone = board.get_state()[0][0];
    let bot_right_stone = board.get_state()[3][3];

    //White
    assert_eq!(top_left_stone, Tile::White);
    //Black
    assert_eq!(bot_right_stone, Tile::Black);
}

#[test]
fn valid_movement_passive_1()
{
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::White, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    let list = b.get_state()[1][2].get_possible_moves(&b, false, (1, 2));

    for i in &list
    {
        println!("({}, {})", i.0, i.1);
    }

    assert!(list.contains(&(2, 2)));
    assert!(list.contains(&(3, 2)));

    assert!(!list.contains(&(0, 2)));
}


#[test]
fn valid_movement_passive_2()
{
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::White, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Empty]
    ];

    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    let list = b.get_state()[2][2].get_possible_moves(&b, false, (2, 2));

    assert!(list.contains(&(1, 2)));

    assert!(!list.contains(&(0, 2)));
    assert!(!list.contains(&(2, 2)));
    assert!(!list.contains(&(3, 2)));
}


#[test]
fn valid_movement_passive_3()
{
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::White, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    let list = b.get_state()[1][2].get_possible_moves(&b, false, (1, 2));

    assert!(!list.contains(&(1, 2)));
    assert!(!list.contains(&(0, 2)));
    assert!(!list.contains(&(2, 2)));
    assert!(!list.contains(&(3, 2)));
}

#[test]
fn valid_movement_locked_passive_1()
{
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Black, Tile::Black, Tile::Black],
        [Tile::Empty, Tile::Black, Tile::White, Tile::Black],
        [Tile::Empty, Tile::Black, Tile::Black, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    let list = b.get_state()[1][2].get_possible_moves(&b, false, (1, 2));

    assert!(list.is_empty());
}

#[test]
fn valid_movement_locked_passive_2()
{
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Black, Tile::Black, Tile::Black],
        [Tile::Black, Tile::Empty, Tile::White, Tile::Black],
        [Tile::Empty, Tile::Black, Tile::Black, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    let list = b.get_state()[1][2].get_possible_moves(&b, false, (1, 2));

    assert_eq!(list[0], (1, 1));
    assert_eq!(list.len(), 1);
}

#[test]
fn valid_movement_locked_aggressive_1()
{
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Black, Tile::Black, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];
    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    let list = b.get_state()[0][3].get_possible_moves(&b, true, (0, 3));

    for l in &list
    {
        println!("posible: {} {}", l.0, l.1);
    }
    assert!(list.contains(&(1, 2)));
    assert!(list.contains(&(2, 1)));
    assert!(list.len() == 2);
}
#[test]
fn valid_movement_locked_aggressive_2()
{
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::White, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::White, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    let list = b.get_state()[0][3].get_possible_moves(&b, true, (0, 3));

    for l in &list
    {
        println!("posible: {} {}", l.0, l.1);
    }
    assert!(list.contains(&(1, 3)));
    assert!(list.contains(&(2, 3)));
    assert_eq!(list.len(), 2);
}
#[test]
fn valid_movement_locked_aggressive_3()
{
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::White, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::White, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    let list = b.get_state()[0][3].get_possible_moves(&b, true, (0, 3));

    for l in &list
    {
        println!("posible: {} {}", l.0, l.1);
    }
    assert_eq!(list.len(), 0);
}

#[test]
fn valid_movement_locked_aggressive_4()
{

    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Black, Tile::White, Tile::White],
        [Tile::Empty, Tile::Black, Tile::White, Tile::White],
        [Tile::Empty, Tile::Black, Tile::Black, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    let list = b.get_state()[0][3].get_possible_moves(&b, true, (0, 3));

    for l in &list
    {
        println!("posible: {} {}", l.0, l.1);
    }
    assert_eq!(list.len(), 0);
}

#[test]
fn check_if_stones_update()
{
    let boardstate: [[Tile; 4]; 4] = [[Tile::empty(); 4]; 4];

    let mut b = Board::new_board(Color::Black, Color::Black);

    b.set_state(boardstate);
}