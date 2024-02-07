use super::game_instance::Game;
use crate::rules::{
    game_board::Board,
    game_tile::Tile,
};

#[cfg(test)]
#[test]
fn state_init() {
    use crate::rules::game_tile::Tile;

    let board = Board::new_board(Tile::Black, Tile::Black);

    let top_left_stone = board.get_state()[0][0];
    let bot_right_stone = board.get_state()[3][3];

    //White
    assert_eq!(top_left_stone, Tile::White);
    //Black
    assert_eq!(bot_right_stone, Tile::Black);
}

#[test]
fn valid_movement_passive_1() {
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::White, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let mut b = Board::new_board(Tile::Black, Tile::Black);
    b.set_state(&boardstate);

    let list = Tile::get_possible_moves(&b, false, (1, 2));

    for i in &list {
        println!("({}, {})", i.0, i.1);
    }

    assert!(list.contains(&(2, 2)));
    assert!(list.contains(&(3, 2)));

    assert!(!list.contains(&(0, 2)));
}

#[test]
fn valid_movement_passive_2() {
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::White, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Empty],
    ];

    let mut b = Board::new_board(Tile::Black, Tile::Black);
    b.set_state(&boardstate);

    let list = Tile::get_possible_moves(&b, false, (2, 2));

    assert!(list.contains(&(1, 2)));

    assert!(!list.contains(&(0, 2)));
    assert!(!list.contains(&(2, 2)));
    assert!(!list.contains(&(3, 2)));
}

#[test]
fn valid_movement_passive_3() {
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::White, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let mut b = Board::new_board(Tile::Black, Tile::Black);
    b.set_state(&boardstate);

    let list = Tile::get_possible_moves(&b, false, (1, 2));

    assert!(!list.contains(&(1, 2)));
    assert!(!list.contains(&(0, 2)));
    assert!(!list.contains(&(2, 2)));
    assert!(!list.contains(&(3, 2)));
}

#[test]
fn valid_movement_locked_passive_1() {
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Black, Tile::Black, Tile::Black],
        [Tile::Empty, Tile::Black, Tile::White, Tile::Black],
        [Tile::Empty, Tile::Black, Tile::Black, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let mut b = Board::new_board(Tile::Black, Tile::Black);
    b.set_state(&boardstate);

    let list = Tile::get_possible_moves(&b, false, (1, 2));

    assert!(list.is_empty());
}

#[test]
fn valid_movement_locked_passive_2() {
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Black, Tile::Black, Tile::Black],
        [Tile::Black, Tile::Empty, Tile::White, Tile::Black],
        [Tile::Empty, Tile::Black, Tile::Black, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let mut b = Board::new_board(Tile::Black, Tile::Black);
    b.set_state(&boardstate);

    let list = Tile::get_possible_moves(&b, false, (1, 2));

    assert_eq!(list[0], (1, 1));
    assert_eq!(list.len(), 1);
}

#[test]
fn valid_movement_locked_aggressive_1() {
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Black, Tile::Black, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];
    let mut b = Board::new_board(Tile::Black, Tile::Black);
    b.set_state(&boardstate);

    let list = Tile::get_possible_moves(&b, true, (0, 3));

    for l in &list {
        println!("posible: {} {}", l.0, l.1);
    }
    assert!(list.contains(&(1, 2)));
    assert!(list.contains(&(2, 1)));
    assert!(list.len() == 2);
}
#[test]
fn valid_movement_locked_aggressive_2() {
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::White, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::White, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let mut b = Board::new_board(Tile::Black, Tile::Black);
    b.set_state(&boardstate);

    let list = Tile::get_possible_moves(&b, true, (0, 3));

    for l in &list {
        println!("posible: {} {}", l.0, l.1);
    }
    assert!(list.contains(&(1, 3)));
    assert!(list.contains(&(2, 3)));
    assert_eq!(list.len(), 2);
}
#[test]
fn valid_movement_locked_aggressive_3() {
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::White, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::White, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let mut b = Board::new_board(Tile::Black, Tile::Black);
    b.set_state(&boardstate);

    let list = Tile::get_possible_moves(&b, true, (0, 3));

    for l in &list {
        println!("posible: {} {}", l.0, l.1);
    }
    assert_eq!(list.len(), 0);
}

#[test]
fn valid_movement_locked_aggressive_4() {
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Black, Tile::White, Tile::White],
        [Tile::Empty, Tile::Black, Tile::White, Tile::White],
        [Tile::Empty, Tile::Black, Tile::Black, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let mut b = Board::new_board(Tile::Black, Tile::Black);
    b.set_state(&boardstate);

    let list = Tile::get_possible_moves(&b, true, (0, 3));

    for l in &list {
        println!("posible: {} {}", l.0, l.1);
    }
    assert_eq!(list.len(), 0);
}

#[test]
fn check_if_stones_update() {
    let boardstate: [[Tile; 4]; 4] = [[Tile::empty(); 4]; 4];

    let mut b = Board::new_board(Tile::Black, Tile::Black);
    let b2 = Board::new_board(Tile::Black, Tile::Black);

    assert_eq!(b.get_state(), b2.get_state());

    b.set_state(&boardstate);

    assert_ne!(b.get_state(), b2.get_state());
}

#[test]
fn check_winner() {
    let boardstate_none: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Black, Tile::White, Tile::White],
        [Tile::Empty, Tile::Black, Tile::White, Tile::White],
        [Tile::Empty, Tile::Black, Tile::Black, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_none2: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_white: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::White, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::White, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_black: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Black, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Black, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Black, Tile::Black, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let mut bn: Board = Board::new_board(Tile::Black, Tile::Black);
    bn.set_state(&boardstate_none);

    let mut bn2: Board = Board::new_board(Tile::Black, Tile::Black);
    bn2.set_state(&boardstate_none2);

    let mut bw: Board = Board::new_board(Tile::Black, Tile::Black);
    bw.set_state(&boardstate_white);

    let mut bb: Board = Board::new_board(Tile::Black, Tile::Black);
    bb.set_state(&boardstate_black);

    assert_eq!(Board::check_winner(&bn), None);
    assert_eq!(Board::check_winner(&bn2), None);

    assert_eq!(Board::check_winner(&bw), Some(Tile::White));
    assert_eq!(Board::check_winner(&bb), Some(Tile::Black));
}

#[test]
fn movement_passive() {
    let mut b = Board::new_board(Tile::Black, Tile::Black);
    let b2 = Board::new_board(Tile::Black, Tile::Black);

    //Ful lösning men eh
    assert!(Tile::passive_move(&mut b, (0, 3), (2, 3)));
    assert_ne!(b.get_state(), b2.get_state());
}

#[test]
fn movement_passive_2() {
    let mut b = Board::new_board(Tile::Black, Tile::Black);

    //Ful lösning men eh
    assert!(!Tile::passive_move(&mut b, (0, 3), (3, 3)));
}

#[test]
fn movement_aggressive_1_step_push() {
    let mut b = Board::new_board(Tile::Black, Tile::Black);
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::White, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_next: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];
    b.set_state(&boardstate);

    let mut b2 = Board::new_board(Tile::Black, Tile::Black);
    b2.set_state(&boardstate);


    assert!(Tile::get_possible_moves(&b, true, (0, 2)).contains(&(0, 1)));

    assert!(Tile::aggressive_move(&mut b, (0, 2), (0, 3)));

    println!("{}", b.fancy_print());
    println!("{}", b2.fancy_print());

    assert_eq!(*b.get_state(), boardstate_next);
    assert_ne!(b2.get_state(), b.get_state());
}

#[test]
fn movement_aggressive_2_step_push_1() {
    let mut b = Board::new_board(Tile::Black, Tile::Black);
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::White, Tile::Black, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_next: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];
    b.set_state(&boardstate);

    let mut b2 = Board::new_board(Tile::Black, Tile::Black);
    b2.set_state(&boardstate);

    Tile::aggressive_move(&mut b, (0, 1), (0, 3));

    assert_eq!(*b.get_state(), boardstate_next);
    assert_ne!(b2.get_state(), b.get_state());

    println!("{:#?}", *b.get_state());
    println!("{:#?}", *b2.get_state());
}

#[test]
fn movement_aggressive_2_step_push_2() {
    let mut b = Board::new_board(Tile::Black, Tile::Black);
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::White, Tile::Empty, Tile::Black],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_next: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];
    b.set_state(&boardstate);

    let mut b2 = Board::new_board(Tile::Black, Tile::Black);
    b2.set_state(&boardstate);

    Tile::aggressive_move(&mut b, (0, 1), (0, 3));

    assert_eq!(*b.get_state(), boardstate_next);
    assert_ne!(b2.get_state(), b.get_state());

    println!("{:#?}", *b.get_state());
    println!("{:#?}", *b2.get_state());
}

#[test]
fn movement_aggressive_1_step_push_2() {
    let mut b = Board::new_board(Tile::Black, Tile::Black);
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Black, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::White, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_next: [[Tile; 4]; 4] = [
        [Tile::White, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];
    b.set_state(&boardstate);

    let mut b2 = Board::new_board(Tile::Black, Tile::Black);
    b2.set_state(&boardstate);

    Tile::aggressive_move(&mut b, (1, 1), (0, 0));

    assert_eq!(*b.get_state(), boardstate_next);
    assert_ne!(b2.get_state(), b.get_state());

    println!("{:#?}", *b.get_state());
    println!("{:#?}", *b2.get_state());
}

#[test]
fn max_movement() {
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
fn movement_aggressive_nopush() {
    let mut b = Board::new_board(Tile::Black, Tile::Black);
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::White, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_next: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];
    b.set_state(&boardstate);

    let mut b2 = Board::new_board(Tile::Black, Tile::Black);
    b2.set_state(&boardstate);

    assert!(Tile::aggressive_move(&mut b, (0, 1), (0, 3)));

    assert_eq!(*b.get_state(), boardstate_next);
    assert_ne!(b2.get_state(), b.get_state());

    println!("{:#?}", *b.get_state());
    println!("{:#?}", *b2.get_state());
}

#[test]
fn movement_aggressive_3_0() {
    let mut b = Board::new_board(Tile::Black, Tile::Black);
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::White, Tile::Empty, Tile::Empty],
        [Tile::Black, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_next: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::White, Tile::Empty, Tile::Empty, Tile::Empty],
    ];
    b.set_state(&boardstate);

    let mut b2 = Board::new_board(Tile::Black, Tile::Black);
    b2.set_state(&boardstate);

    assert!(Tile::aggressive_move(&mut b, (2, 1), (3, 0)));

    assert_eq!(*b.get_state(), boardstate_next);
    assert_ne!(b2.get_state(), b.get_state());

    println!("{:#?}", *b.get_state());
    println!("{:#?}", *b2.get_state());
}

#[test]
fn movement_aggressive_3_0_nopush() {
    let mut b = Board::new_board(Tile::Black, Tile::Black);
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::White, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_next: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::White, Tile::Empty, Tile::Empty, Tile::Empty],
    ];
    b.set_state(&boardstate);

    let mut b2 = Board::new_board(Tile::Black, Tile::Black);
    b2.set_state(&boardstate);

    assert!(Tile::aggressive_move(&mut b, (2, 1), (3, 0)));

    assert_eq!(*b.get_state(), boardstate_next);
    assert_ne!(b2.get_state(), b.get_state());

    println!("{:#?}", *b.get_state());
    println!("{:#?}", *b2.get_state());
}

#[test]
fn movement_aggressive_3_0_slightpush() {
    let mut b = Board::new_board(Tile::Black, Tile::Black);
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::White, Tile::Empty],
        [Tile::Empty, Tile::Black, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_next: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::White,  Tile::Empty, Tile::Empty],
        [Tile::Black, Tile::Empty, Tile::Empty, Tile::Empty],
    ];
    b.set_state(&boardstate);

    let mut b2 = Board::new_board(Tile::Black, Tile::Black);
    b2.set_state(&boardstate);

    assert!(Tile::aggressive_move(&mut b, (1, 2), (2, 1)));

    println!("{}", b.fancy_print());
    println!("{}", b2.fancy_print());

    assert_eq!(*b.get_state(), boardstate_next);
    assert_ne!(b2.get_state(), b.get_state());
}

#[test]
fn movement_aggressive_0_0_push_1_step() {
    let mut b = Board::new_board(Tile::Black, Tile::Black);
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Black, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::White, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_next: [[Tile; 4]; 4] = [
        [Tile::White, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];
    b.set_state(&boardstate);

    let mut b2 = Board::new_board(Tile::Black, Tile::Black);
    b2.set_state(&boardstate);

    assert!(Tile::aggressive_move(&mut b, (1, 1), (0, 0)));

    assert_eq!(*b.get_state(), boardstate_next);
    assert_ne!(b2.get_state(), b.get_state());

    println!("{:#?}", *b.get_state());
    println!("{:#?}", *b2.get_state());
}

#[test]
fn movement_aggressive_0_0_push_2_steps() {
    let mut b = Board::new_board(Tile::Black, Tile::Black);
    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Black, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::White, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_next: [[Tile; 4]; 4] = [
        [Tile::White, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];
    b.set_state(&boardstate);

    let mut b2 = Board::new_board(Tile::Black, Tile::Black);
    b2.set_state(&boardstate);

    assert!(Tile::aggressive_move(&mut b, (2, 2), (0, 0)));

    assert_eq!(*b.get_state(), boardstate_next);
    assert_ne!(b2.get_state(), b.get_state());

    println!("{:#?}", *b.get_state());
    println!("{:#?}", *b2.get_state());
}

#[test]
fn get_board_from_game_instance_test() {
    let mut g = Game::new_game();
    let b = g.get_board(Tile::Black, Tile::White).unwrap();

    assert_eq!(b.get_color(), Tile::White);
    assert_eq!(b.get_home(), Tile::Black);
}

#[test]
fn movement_one_in_each_dir_1_step_passive() {
    let mut b = Board::new_board(Tile::Black, Tile::Black);

    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::White, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    b.set_state(&boardstate);

    let directions = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (-1, 1),
        (1, 1),
        (1, -1),
    ];

    for d in directions {
        assert!(Tile::is_valid(
            b.get_state(),
            (2, 2),
            (2 + d.0, 2 + d.1),
            &1,
            false,
            (&d.0, &d.1)
        ));
    }
}

#[test]
fn movement_one_in_each_dir_1_step_aggressive() {
    let mut b = Board::new_board(Tile::Black, Tile::Black);

    let boardstate: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Black, Tile::Black, Tile::Black],
        [Tile::Empty, Tile::Black, Tile::White, Tile::Black],
        [Tile::Empty, Tile::Black, Tile::Black, Tile::Black],
    ];

    b.set_state(&boardstate);

    let directions = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (-1, 1),
        (1, 1),
        (1, -1),
    ];

    for d in directions {
        assert!(Tile::is_valid(
            b.get_state(),
            (2, 2),
            (2 + d.0, 2 + d.1),
            &1,
            true,
            (&d.0, &d.1)
        ));
    }
}

#[test]
fn movement_one_in_each_dir_2_step_passive() {
    let mut b_tl = Board::new_board(Tile::Black, Tile::Black);
    let mut b_tr = Board::new_board(Tile::Black, Tile::White);
    let mut b_bl = Board::new_board(Tile::White, Tile::Black);
    let mut b_br = Board::new_board(Tile::White, Tile::White);

    let boardstate_tl: [[Tile; 4]; 4] = [
        [Tile::White, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_tr: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_bl: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::White, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_br: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
    ];

    b_tl.set_state(&boardstate_tl);
    b_tr.set_state(&boardstate_tr);
    b_bl.set_state(&boardstate_bl);
    b_br.set_state(&boardstate_br);

    let dir_tl = [(1, 0), (1, 1), (0, 1)];
    let dir_tr = [(1, 0), (1, -1), (0, -1)];
    let dir_bl = [(-1, 0), (-1, 1), (0, 1)];
    let dir_br = [(-1, 0), (-1, -1), (0, -1)];

    for d in dir_tl {
        assert!(Tile::is_valid(
            b_tl.get_state(),
            (0, 0),
            (2 + d.0, 2 + d.1),
            &2,
            false,
            (&d.0, &d.1)
        ));
    }

    for d in dir_tr {
        assert!(Tile::is_valid(
            b_tr.get_state(),
            (0, 3),
            (2 + d.0, 2 + d.1),
            &2,
            false,
            (&d.0, &d.1)
        ));
    }

    for d in dir_bl {
        assert!(Tile::is_valid(
            b_bl.get_state(),
            (3, 0),
            (2 + d.0, 2 + d.1),
            &2,
            false,
            (&d.0, &d.1)
        ));
    }

    for d in dir_br {
        assert!(Tile::is_valid(
            b_br.get_state(),
            (3, 3),
            (2 + d.0, 2 + d.1),
            &2,
            false,
            (&d.0, &d.1)
        ));
    }
}

#[test]
fn movement_one_in_each_dir_2_step_aggr() {
    let mut b_tl = Board::new_board(Tile::Black, Tile::Black);
    let mut b_tr = Board::new_board(Tile::Black, Tile::White);
    let mut b_bl = Board::new_board(Tile::White, Tile::Black);
    let mut b_br = Board::new_board(Tile::White, Tile::White);

    let boardstate_tl: [[Tile; 4]; 4] = [
        [Tile::White, Tile::Empty, Tile::Black, Tile::Empty],
        [Tile::Empty, Tile::Black, Tile::Empty, Tile::Empty],
        [Tile::Black, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_tr: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_bl: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::White, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let boardstate_br: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
    ];

    b_tl.set_state(&boardstate_tl);
    b_tr.set_state(&boardstate_tr);
    b_bl.set_state(&boardstate_bl);
    b_br.set_state(&boardstate_br);

    let dir_tl = [(1, 0), (1, 1), (0, 1)];
    let dir_tr = [(1, 0), (1, -1), (0, -1)];
    let dir_bl = [(-1, 0), (-1, 1), (0, 1)];
    let dir_br = [(-1, 0), (-1, -1), (0, -1)];

    for d in dir_tl {
        assert!(Tile::is_valid(
            b_tl.get_state(),
            (0, 0),
            (2 + d.0, 2 + d.1),
            &2,
            true,
            (&d.0, &d.1)
        ));
    }

    for d in dir_tr {
        assert!(Tile::is_valid(
            b_tr.get_state(),
            (0, 3),
            (2 + d.0, 2 + d.1),
            &2,
            true,
            (&d.0, &d.1)
        ));
    }

    for d in dir_bl {
        assert!(Tile::is_valid(
            b_bl.get_state(),
            (3, 0),
            (2 + d.0, 2 + d.1),
            &2,
            true,
            (&d.0, &d.1)
        ));
    }

    for d in dir_br {
        assert!(Tile::is_valid(
            b_br.get_state(),
            (3, 3),
            (2 + d.0, 2 + d.1),
            &2,
            true,
            (&d.0, &d.1)
        ));
    }
}

#[test]
fn leapfrog_1() {
    let state: [[Tile; 4]; 4] = [
        [Tile::White, Tile::Empty, Tile::Empty, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Empty],
        [Tile::Black, Tile::White, Tile::Empty, Tile::Empty],
        [Tile::Black, Tile::Empty, Tile::Empty, Tile::White],
    ];

    let target_state: [[Tile; 4]; 4] = [
        [Tile::White, Tile::Empty, Tile::Empty, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::White],
        [Tile::Black, Tile::Empty, Tile::Empty, Tile::White],
    ];

    let mut board = Board::new_board(Tile::Black, Tile::White);
    board.set_state(&state);


    assert!(Tile::aggressive_move(&mut board, (2, 0), (2, 2)));

    assert_eq!(board.get_state(), &target_state);
}

#[test]
fn leapfrog_2() {
    let state: [[Tile; 4]; 4] = [
        [Tile::Black, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::White, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let target_state: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
    ];

    let mut board = Board::new_board(Tile::Black, Tile::White);
    board.set_state(&state);


    assert!(Tile::aggressive_move(&mut board, (0, 0), (2, 2)));

    assert_eq!(board.get_state(), &target_state);
}


#[test]
fn leapfrog_3() {
    let state: [[Tile; 4]; 4] = [
        [Tile::White, Tile::White, Tile::White, Tile::White],
        [Tile::Empty, Tile::Black, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Black, Tile::Black, Tile::Black, Tile::Empty],
    ];

    let mut board = Board::new_board(Tile::Black, Tile::White);
    board.set_state(&state);

    let move_list = Tile::get_possible_moves(&mut board, true, (0, 1));
    println!("{:?}", move_list);
    assert!(!move_list.contains(&(2, 1)));
}


#[test]
fn add_player_test() {
    let mut g = Game::new_game();

    assert!(g.add_player(String::from("Testplayer_b")));
    assert!(!g.add_player(String::from("Testplayer_b")));

    assert!(g.add_player(String::from("Testplayer_w")));

    assert!(!g.add_player(String::from("spectator")));

    let players = g.get_players();

    assert_eq!(players.0, String::from("Testplayer_b"));
    assert_eq!(players.1, String::from("Testplayer_w"));
}

#[test]
fn is_empty_test() {
    assert!(Tile::is_empty(Tile::empty()));
    assert!(!Tile::is_empty(Tile::white()));
    assert!(!Tile::is_empty(Tile::black()));
}

#[test]
fn diagonal_push_1(){
    let state: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::White, Tile::Empty],
        [Tile::Black, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Black, Tile::Empty, Tile::Empty, Tile::Black],
    ];

    let target_state: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Empty],
        [Tile::Black, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Black],
    ];

    /*
    We want this:
    [ ][ ][ ][ ]      [ ][ ][ ][W]
    [ ][ ][W][ ]   => [ ][ ][B][ ]
    [B][ ][ ][ ]      [B][ ][ ][ ]
    [B][ ][ ][B]      [ ][ ][ ][B]

     */

    let mut board = Board::new_board(Tile::Black, Tile::White);
    board.set_state(&state);

    assert!(Tile::aggressive_move(&mut board, (3, 0), (1, 2))); 
    assert_eq!(board.get_state(), &target_state);
}


#[test]
fn diagonal_push_2(){
    let state: [[Tile; 4]; 4] = [
        [Tile::White, Tile::Empty, Tile::White, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Black],
        [Tile::Empty, Tile::Black, Tile::Empty, Tile::White],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Black],
    ];

    let target_state: [[Tile; 4]; 4] = [
        [Tile::White, Tile::Empty, Tile::White, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Black],
        [Tile::Empty, Tile::White, Tile::Empty, Tile::White],
        [Tile::Black, Tile::Empty, Tile::Empty, Tile::Black],
    ];

    /*
    We want this:
    [W][ ][W][W]      [W][ ][W][ ]
    [ ][ ][ ][B]   => [ ][ ][ ][B]
    [ ][B][ ][W]      [ ][W][ ][W]
    [ ][ ][ ][B]      [B][ ][ ][B]

     */

    let mut board = Board::new_board(Tile::Black, Tile::Black);
    board.set_state(&state);

    assert!(Tile::aggressive_move(&mut board, (0, 3), (2, 1))); 
    assert_eq!(board.get_state(), &target_state);
}

#[test]
fn diagonal_push_3(){
    let state: [[Tile; 4]; 4] = [
        [Tile::White, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let target_state: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::White, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Black],
    ];

    /*
    We want this:
    [W][ ][ ][ ]      [ ][ ][ ][ ]
    [ ][ ][ ][ ]   => [ ][ ][ ][ ]
    [ ][ ][B][ ]      [ ][ ][W][ ]
    [ ][ ][ ][ ]      [ ][ ][ ][B]

     */

    let mut board = Board::new_board(Tile::Black, Tile::Black);
    board.set_state(&state);

    assert!(Tile::aggressive_move(&mut board, (0, 0), (2, 2))); 
    assert_eq!(board.get_state(), &target_state);
}

#[test]
fn diagonal_push_4(){
    let state: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
    ];

    let target_state: [[Tile; 4]; 4] = [
        [Tile::Black, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::White, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let mut board = Board::new_board(Tile::Black, Tile::Black);
    board.set_state(&state);

    /*
    We want this:
    [ ][ ][ ][ ]      [B][ ][ ][ ]
    [ ][B][ ][ ]   => [ ][W][ ][ ]
    [ ][ ][ ][ ]      [ ][ ][ ][ ]
    [ ][ ][ ][W]      [ ][ ][ ][ ]

     */

    assert!(Tile::aggressive_move(&mut board, (3, 3), (1, 1))); 
    assert_eq!(board.get_state(), &target_state);
}

#[test]
fn diagonal_push_5(){
    let state: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Black, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
    ];

    let target_state: [[Tile; 4]; 4] = [
        [Tile::Black, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::White, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let mut board = Board::new_board(Tile::Black, Tile::Black);
    board.set_state(&state);

    assert!(Tile::aggressive_move(&mut board, (3, 3), (1, 1))); 
    assert_eq!(board.get_state(), &target_state);
}

#[test]
fn diagonal_push_1_step(){
    let state: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Black, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::White],
    ];

    let target_state: [[Tile; 4]; 4] = [
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Black, Tile::Empty, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::White, Tile::Empty],
        [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    ];

    let mut board = Board::new_board(Tile::Black, Tile::Black);
    board.set_state(&state);

    assert!(Tile::aggressive_move(&mut board, (3, 3), (2, 2))); 
    assert_eq!(board.get_state(), &target_state);
}