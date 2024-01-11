use crate::rules::game_rules::*;

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
    let b2 = Board::new_board(Color::Black, Color::Black);

    assert_eq!(b.get_state(), b2.get_state());

    b.set_state(boardstate);

    
    assert_ne!(b.get_state(), b2.get_state());
}

#[test]
fn check_winner()
{
    let boardstate_none: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Black, Tile::White, Tile::White],
        [Tile::Empty, Tile::Black, Tile::White, Tile::White],
        [Tile::Empty, Tile::Black, Tile::Black, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let boardstate_none2: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let boardstate_white: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::White, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::White, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let boardstate_black: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Black, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Black, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Black, Tile::Black, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let mut bn: Board = Board::new_board(Color::Black, Color::Black);
    bn.set_state(boardstate_none);

    let mut bn2: Board = Board::new_board(Color::Black, Color::Black);
    bn2.set_state(boardstate_none2);

    let mut bw: Board = Board::new_board(Color::Black, Color::Black);
    bw.set_state(boardstate_white);

    let mut bb: Board = Board::new_board(Color::Black, Color::Black);
    bb.set_state(boardstate_black);

    assert_eq!(Board::check_winner(&bn), None);
    assert_eq!(Board::check_winner(&bn2), None);

    assert_eq!(Board::check_winner(&bw), Some(Color::White));
    assert_eq!(Board::check_winner(&bb), Some(Color::Black));
}

#[test]
fn movement_passive()
{
    let mut b = Board::new_board(Color::Black, Color::Black);
    let b2 = Board::new_board(Color::Black, Color::Black);

    //Ful lösning men eh
    let (pos, diff, _) = b.to_owned().get_state()[0][3].passive_move(&mut b, (0, 3), (2, 3));
    assert!(pos);
    assert!(diff == (2, 0));
    assert_ne!(b.get_state(), b2.get_state());
}

#[test]
fn movement_passive_2()
{
    let mut b = Board::new_board(Color::Black, Color::Black);

    //Ful lösning men eh
    let (pos, _, _) = b.to_owned().get_state()[0][3].passive_move(&mut b, (0, 3), (3, 3));
    assert!(!pos);
}

#[test]
fn movement_aggressive_1_step_push()
{
    let mut b = Board::new_board(Color::Black, Color::Black);
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::White, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let boardstate_next: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];
    b.set_state(boardstate);

    let mut b2 = Board::new_board(Color::Black, Color::Black);
    b2.set_state(boardstate);

    assert!(b.get_state()[0][2].get_possible_moves(&b, true, (0, 2)).contains(&(0, 1)));

    print!("POSSIBLE");

    b.to_owned().get_state()[0][2].aggressive_move(&mut b, (0, 2), (0, 1), Color::White);

    assert_eq!(*b.get_state(), boardstate_next);
    assert_ne!(b2.get_state(), b.get_state());
    println!("{:#?}", *b.get_state());
    println!("{:#?}", *b2.get_state());
}


#[test]
fn movement_aggressive_2_step_push_1()
{
    let mut b = Board::new_board(Color::Black, Color::Black);
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::White, Tile::Black, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let boardstate_next: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];
    b.set_state(boardstate);

    let mut b2 = Board::new_board(Color::Black, Color::Black);
    b2.set_state(boardstate);

    b.to_owned().get_state()[0][1].aggressive_move(&mut b, (0, 1), (0, 2), Color::White);

    assert_eq!(*b.get_state(), boardstate_next);
    assert_ne!(b2.get_state(), b.get_state());
    println!("{:#?}", *b.get_state());
    println!("{:#?}", *b2.get_state());
}

#[test]
fn movement_aggressive_2_step_push_2()
{
    let mut b = Board::new_board(Color::Black, Color::Black);
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::White, Tile::Empty, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let boardstate_next: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];
    b.set_state(boardstate);

    let mut b2 = Board::new_board(Color::Black, Color::Black);
    b2.set_state(boardstate);

    b.to_owned().get_state()[0][1].aggressive_move(&mut b, (0, 1), (0, 2), Color::White);

    assert_eq!(*b.get_state(), boardstate_next);
    assert_ne!(b2.get_state(), b.get_state());
    println!("{:#?}", *b.get_state());
    println!("{:#?}", *b2.get_state());
}


#[test]
fn movement_aggressive_1_step_push_2()
{
    let mut b = Board::new_board(Color::Black, Color::Black);
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Black, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::White, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let boardstate_next: [[Tile; 4]; 4] = [
        [Tile::White, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];
    b.set_state(boardstate);

    let mut b2 = Board::new_board(Color::Black, Color::Black);
    b2.set_state(boardstate);

    b.to_owned().get_state()[1][1].aggressive_move(&mut b, (1, 1), (-1, -1), Color::White);

    assert_eq!(*b.get_state(), boardstate_next);
    assert_ne!(b2.get_state(), b.get_state());
    println!("{:#?}", *b.get_state());
    println!("{:#?}", *b2.get_state());
}



#[test]
fn max_movement()
{
    let diff: (i8, i8) = (-2, 0);
    assert_eq!(diff.0.abs().max(diff.1.abs()), 2);
    println!("{:#?} {}", diff, diff.0.abs().max(diff.1.abs()));
    
    let diff2: (i8, i8) = (-2, 2);
    assert_eq!(diff2.0.abs().max(diff2.1.abs()), 2);
    println!("{:#?} {}", diff2, diff2.0.abs().max(diff2.1.abs()));

    
    let diff3: (i8, i8) = (-1, 0);
    assert_eq!(diff3.0.abs().max(diff3.1.abs()), 1);
    println!("{:#?} {}", diff3, diff3.0.abs().max(diff3.1.abs()));

    
    let diff4: (i8, i8) = (0, 2);
    assert_eq!(diff4.0.abs().max(diff4.1.abs()), 2);
    println!("{:#?} {}", diff4, diff4.0.abs().max(diff4.1.abs()));

    
    let diff5: (i8, i8) = (2, -2);
    assert_eq!(diff5.0.abs().max(diff5.1.abs()), 2);
    println!("{:#?} {}", diff5, diff5.0.abs().max(diff5.1.abs()));
}


#[test]
fn movement_aggressive_nopush()
{
    let mut b = Board::new_board(Color::Black, Color::Black);
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::White, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];

    let boardstate_next: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty]
    ];
    b.set_state(boardstate);

    let mut b2 = Board::new_board(Color::Black, Color::Black);
    b2.set_state(boardstate);

    assert!(b.to_owned().get_state()[0][1].aggressive_move(&mut b, (0, 1), (0, 2), Color::White));
    assert_eq!(*b.get_state(), boardstate_next);
    assert_ne!(b2.get_state(), b.get_state());
    println!("{:#?}", *b.get_state());
    println!("{:#?}", *b2.get_state());
}