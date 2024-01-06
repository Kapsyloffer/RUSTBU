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
fn valid_movement_passive_1()
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
fn valid_movement_passive_2()
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

    assert!(list.contains(&(1, 2)));

    assert!(!list.contains(&(0, 2)));
    assert!(!list.contains(&(2, 2)));
    assert!(!list.contains(&(3, 2)));
}


#[test]
fn valid_movement_passive_3()
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

    assert!(!list.contains(&(1, 2)));
    assert!(!list.contains(&(0, 2)));
    assert!(!list.contains(&(2, 2)));
    assert!(!list.contains(&(3, 2)));
}

#[test]
fn valid_movement_locked_passive_1()
{
    let whitestone  = Some(Stone::new(Color::White, (1, 2)));
    let blackstone1 = Some(Stone::new(Color::Black, (0, 2)));
    let blackstone2 = Some(Stone::new(Color::Black, (0, 1)));
    let blackstone3 = Some(Stone::new(Color::Black, (1, 1)));
    let blackstone4 = Some(Stone::new(Color::Black, (1, 1)));
    let blackstone5 = Some(Stone::new(Color::Black, (2, 2)));
    let blackstone6 = Some(Stone::new(Color::Black, (2, 3)));
    let blackstone7 = Some(Stone::new(Color::Black, (1, 3)));
    let blackstone8 = Some(Stone::new(Color::Black, (0, 3)));

    let boardstate: Vec<Vec<Option<Stone>>> = vec![
        vec![None, blackstone2, blackstone1, blackstone8],
        vec![None, blackstone3, whitestone , blackstone7],
        vec![None, blackstone4, blackstone5, blackstone6],
        vec![None, None, None, None],
    ];

    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    let list = whitestone.unwrap().get_possible_moves(&b, false);

    assert!(list.is_empty());
}

#[test]
fn valid_movement_locked_passive_2()
{
    let whitestone  = Some(Stone::new(Color::White, (1, 2)));
    let blackstone1 = Some(Stone::new(Color::Black, (0, 2)));
    let blackstone2 = Some(Stone::new(Color::Black, (0, 1)));
    let blackstone3 = Some(Stone::new(Color::Black, (1, 0)));
    let blackstone4 = Some(Stone::new(Color::Black, (1, 1)));
    let blackstone5 = Some(Stone::new(Color::Black, (2, 2)));
    let blackstone6 = Some(Stone::new(Color::Black, (2, 3)));
    let blackstone7 = Some(Stone::new(Color::Black, (1, 3)));
    let blackstone8 = Some(Stone::new(Color::Black, (0, 3)));

    let boardstate: Vec<Vec<Option<Stone>>> = vec![
        vec![None, blackstone2, blackstone1, blackstone8],
        vec![blackstone3, None, whitestone , blackstone7],
        vec![None, blackstone4, blackstone5, blackstone6],
        vec![None, None, None, None],
    ];

    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    let list = whitestone.unwrap().get_possible_moves(&b, false);

    assert_eq!(list[0], (1, 1));
    assert_eq!(list.len(), 1);
}


#[test]
fn valid_movement_locked_aggressive_1()
{
    let whitestone   = Some(Stone::new(Color::White, (0, 3)));
    let blackstone1  = Some(Stone::new(Color::Black, (0, 2)));
    let blackstone2  = Some(Stone::new(Color::Black, (1, 2)));
    let blackstone3  = Some(Stone::new(Color::Black, (1, 3)));
    let blackstone4  = Some(Stone::new(Color::Black, (0, 1)));
    let blackstone5  = Some(Stone::new(Color::Black, (2, 3)));

    let boardstate: Vec<Vec<Option<Stone>>> = vec![
        vec![None, blackstone4, blackstone1,    whitestone],
        vec![None, None,        blackstone2,    blackstone3],
        vec![None, None,        None,           blackstone5],
        vec![None, None,        None,           None],
    ];

    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    let list = whitestone.unwrap().get_possible_moves(&b, true);

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
    let whitestone0  = Some(Stone::new(Color::White, (0, 3)));
    let whitestone1  = Some(Stone::new(Color::White, (0, 2)));
    let whitestone2  = Some(Stone::new(Color::White, (1, 2)));

    let blackstone1  = Some(Stone::new(Color::Black, (2, 3)));

    let boardstate: Vec<Vec<Option<Stone>>> = vec![
        vec![None, None, whitestone1, whitestone0],
        vec![None, None, whitestone2, None],
        vec![None, None, None, blackstone1],
        vec![None, None, None, None],
    ];

    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    let list = whitestone0.unwrap().get_possible_moves(&b, true);

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
    let whitestone0  = Some(Stone::new(Color::White, (0, 3)));
    let whitestone1  = Some(Stone::new(Color::White, (0, 2)));
    let whitestone2  = Some(Stone::new(Color::White, (1, 2)));
    let whitestone3  = Some(Stone::new(Color::White, (1, 3)));

    let boardstate: Vec<Vec<Option<Stone>>> = vec![
        vec![None, None, whitestone1, whitestone0],
        vec![None, None, whitestone2, whitestone3],
        vec![None, None, None, None],
        vec![None, None, None, None],
    ];

    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    let list = whitestone0.unwrap().get_possible_moves(&b, true);

    for l in &list
    {
        println!("posible: {} {}", l.0, l.1);
    }
    assert_eq!(list.len(), 0);
}

#[test]
fn valid_movement_locked_aggressive_4()
{
    let whitestone0  = Some(Stone::new(Color::White, (0, 3)));
    let whitestone1  = Some(Stone::new(Color::White, (0, 2)));
    let whitestone2  = Some(Stone::new(Color::White, (1, 2)));
    let whitestone3  = Some(Stone::new(Color::White, (1, 3)));

    let black0  = Some(Stone::new(Color::Black, (0, 1)));
    let black1  = Some(Stone::new(Color::Black, (1, 1)));
    let black2  = Some(Stone::new(Color::Black, (2, 1)));
    let black3  = Some(Stone::new(Color::Black, (2, 2)));
    let black4  = Some(Stone::new(Color::Black, (2, 3)));

    let boardstate: Vec<Vec<Option<Stone>>> = vec![
        vec![None, black0, whitestone1, whitestone0],
        vec![None, black1, whitestone2, whitestone3],
        vec![None, black2, black3, black4],
        vec![None, None, None, None],
    ];

    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    let list = whitestone0.unwrap().get_possible_moves(&b, true);

    for l in &list
    {
        println!("posible: {} {}", l.0, l.1);
    }
    assert_eq!(list.len(), 0);
}

#[test]
fn check_if_stones_update()
{
    let black0  = Some(Stone::new(Color::Black, (0, 0)));
    let black1  = Some(Stone::new(Color::Black, (0, 0)));
    let black2  = Some(Stone::new(Color::Black, (0, 0)));
    let black3  = Some(Stone::new(Color::Black, (0, 0)));
    let black4  = Some(Stone::new(Color::Black, (0, 0)));

    let boardstate: Vec<Vec<Option<Stone>>> = vec![
        vec![None, black0, None, None],
        vec![None, black1, None, None],
        vec![None, black2, black3, black4],
        vec![None, None, None, None],
    ];

    let mut b = Board::new_board(Color::Black, Color::Black);
    b.set_state(boardstate);

    assert_ne!(black0.unwrap().get_pos(), (0, 0));
    assert_ne!(black1.unwrap().get_pos(), (0, 0));
    assert_ne!(black2.unwrap().get_pos(), (0, 0));
    assert_ne!(black3.unwrap().get_pos(), (0, 0));
    assert_ne!(black4.unwrap().get_pos(), (0, 0));
}