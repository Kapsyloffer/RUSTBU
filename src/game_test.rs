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
fn valid_movement_north_and_south_1_passive()
{
    let whitestone = Some(Stone::new(Color::White, (1,2)));
    let blackstone = Some(Stone::new(Color::Black, (0,2)));

    let boardstate: Vec<Vec<Option<Stone>>> = vec![
        vec![None, None, blackstone, None],
        vec![None, None, whitestone, None],
        vec![None, None, None, None],
        vec![None, None, None, None],
    ];

    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    let list = whitestone.unwrap().get_possible_moves(&b, false);

    for i in &list
    {
        println!("({}, {})", i.0, i.1);
    }

    assert!(list.contains(&(2, 2)));
    assert!(list.contains(&(3, 2)));

    assert!(!list.contains(&(0, 2)));
}


#[test]
fn valid_movement_north_and_south_2_passive()
{
    let whitestone  = Some(Stone::new(Color::White, (2, 2)));
    let blackstone  = Some(Stone::new(Color::Black, (0, 2)));
    let blackstone2 = Some(Stone::new(Color::Black, (3, 2)));

    let boardstate: Vec<Vec<Option<Stone>>> = vec![
        vec![None, None, blackstone, None],
        vec![None, None, None, None],
        vec![None, None, whitestone, None],
        vec![None, None, blackstone2, None],
    ];

    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    let list = whitestone.unwrap().get_possible_moves(&b, false);

    for i in &list
    {
        println!("ns2:  ({}, {})", i.0, i.1);
    }

    assert!(list.contains(&(1, 2)));

    assert!(!list.contains(&(0, 2)));
    assert!(!list.contains(&(2, 2)));
    assert!(!list.contains(&(3, 2)));
}


#[test]
fn valid_movement_north_and_south_3_passive()
{
    let whitestone  = Some(Stone::new(Color::White, (1, 2)));
    let blackstone  = Some(Stone::new(Color::Black, (0, 2)));
    let blackstone2 = Some(Stone::new(Color::Black, (2, 2)));

    let boardstate: Vec<Vec<Option<Stone>>> = vec![
        vec![None, None, blackstone, None],
        vec![None, None, whitestone, None],
        vec![None, None, blackstone2, None],
        vec![None, None, None, None],
    ];

    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    let list = whitestone.unwrap().get_possible_moves(&b, false);

    for i in &list
    {
        println!("ns3:  ({}, {})", i.0, i.1);
    }

    assert!(!list.contains(&(1, 2)));
    assert!(!list.contains(&(0, 2)));
    assert!(!list.contains(&(2, 2)));
    assert!(!list.contains(&(3, 2)));
}