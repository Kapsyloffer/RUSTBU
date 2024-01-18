#![allow(unused)]
use core::panic;
use std::fmt::Error;

use rocket::{*, response::content::*, http::{hyper::Response, Cookie}};
use crate::rules::{game_board::{Color, Board}, game_hodler::*, game_instance::Game, game_tile::Tile};

//Skapa en ny game lobby med en random url
//t.ex. rsIa8ZVuA
//Och redirectar direkt till den.
#[get("/new")]
pub fn new_game_instance<'r>(shared: &State<GameHodler>) -> response::Redirect
{
    let g = Game::new_game();
    let mut games_map = shared.games.lock().expect("FAILED TO LOCk");

    let url = Game::generate_url();
    games_map.insert(url.to_owned(), g);

    return response::Redirect::to(format!("/api/game/state/{}", url));
}

//Fetcha en gamestate med associated URL
#[get("/state/<url>")]
pub fn get_game_instance<'r>(url: String, shared: &State<GameHodler>) -> String
{
    let games_map = shared.games.lock().expect("FAILED TO LOCk");
    let g: &Game = games_map.get(&url).unwrap();
    return format!("{:#?}", g);
}

#[get("/make_move/<url>/<p>/<a>")]
pub fn make_move(url: String, p: String, a: String, shared: &State<GameHodler>) -> RawJson<&'static str>
{
    if parse_move(&url, &p, &shared).is_ok() && parse_move(&url, &a, &shared).is_ok()
    {
        if &a.as_bytes()[1] == &p.as_bytes()[1]
        {
            panic!()
        }
        move_rocks(&url, &p, shared);
        move_rocks(&url, &a, shared);
        return RawJson("we good");
    }
    let mut board = shared.games.lock().expect("FAILED TO LOCk").get_mut(&url).unwrap().get_boards();
    RawJson("true")
}

pub fn move_rocks(url: &String, m: &String, shared: &State<GameHodler>) -> Result<(), ()>
{
    //Vi checkar om det är valid
    let parsed_move = parse_move(url, m, shared);

    //If it ain't we kirr
    if parsed_move.is_err(){
        return Err(());
    }

    let (homeside, colour, x1, y1, x2, y2, aggr) = parsed_move.unwrap();

    let mut game = shared.games.lock().expect("Failed to lock in parse moves");
    let mut board = game.get_mut(url).unwrap().get_board(homeside, colour).unwrap().to_owned();
    
    let delta_x = (x2 - x1).abs();
    let delta_y = (y2 - y1).abs();

    //print!("{:#?}", board);

    //print!("x1: {}\ny1: {}\nx2: {}\ny2: {}\nΔx: {}\nΔy: {}\n", x1, y1, x2, y2, delta_x, delta_y);

    //Om vårt move är invalid returnar vi false.
    if !Tile::is_valid(board.get_state(), (x1, y1), (x2, y2), &delta_x.max(delta_y), aggr, (&delta_x, &delta_y))
    {
        return Err(());
    }
    
    //ANNARS KÖR VI
    match aggr
    {
        false => Tile::aggressive_move(&mut board, (x1, y1), (delta_x, delta_y)),
        true => Tile::passive_move(&mut board, (x1, y1), (x2, y2)),
    };

    Ok(())
}

//Denna funktionen tar en string t.ex. BW1131A och parsar den.
pub fn parse_move(url: &String, m: &String, shared: &State<GameHodler>) -> Result<(Color, Color, i8, i8, i8, i8, bool), ()>
{
    let list: Vec<char> = m.to_lowercase().chars().collect();
    //print!("{:#?}", list);
    if list.len() != 7
    {
        return Err(());
    }
    /*
    movestringen ser ut såhär:

    BW1131A

    0: Home_size : B
    1: Board_Colour : W

    2: POS X1 : 1
    3: POS Y1 : 1

    4: POS x2 : 3
    5: POS Y2 : 3

    6: MOVETYPE: A(ggressive)

     */

     // B W 1 1 3 1 A
     // ^
    let homeside = match list[0]
    {
        'b' => Color::Black,
        'w' => Color::White,
        _=> return Err(()),
    };

    // B W 1 1 3 1 A
    //   ^
    let colour = match list[1]
    {
        'b' => Color::Black,
        'w' => Color::White,
        _=> return Err(()),
    };

    let mut game = shared.games.lock().expect("Failed to lock in parse moves");
    let mut board = game.get_mut(url).unwrap().get_board(homeside, colour).unwrap().to_owned();

    // B W 1 1 3 1 A
    //     ^
    let x1: i8 = match list[2].to_digit(4) 
    {
        Some(digit) => digit as i8,
        None => 
        {
            return Err(());
        }
    };
    
    // B W 1 1 3 1 A
    //       ^
    let y1: i8 = match list[3].to_digit(4) 
    {
        Some(digit) => digit as i8,
        None => 
        {
            return Err(());
        }
    };

    // B W 1 1 3 1 A
    //         ^
    let x2: i8 = match list[4].to_digit(4) 
    {
        Some(digit) => digit as i8,
        None => 
        {
            return Err(());
        }
    };

    // B W 1 1 3 1 A
    //           ^
    let y2: i8 = match list[5].to_digit(4) 
    {
        Some(digit) => digit as i8,
        None => 
        {
            return Err(());
        }
    };

    // B W 1 1 3 1 A
    //             ^
    let aggr = match list[6]
    {
        'a' => true,
        'p' => false,
        _=>  return Err(()),
    };

    let delta_x = (x2 - x1).abs();
    let delta_y = (y2 - y1).abs();

    if delta_x == 0 && delta_y == 0
    {
        return Err(());
    }

    Ok((homeside, colour, x1, y1, x2, y2, aggr))
}


//Kolla vilken spelare du är i ett game, om rollen är tom, blir du den rollen.
//I guess used for debugging
#[get("/whoami/<url>")]
pub fn who_am_i(url: String, shared: &State<GameHodler>) -> String
{
    let (mut b, mut w) = shared.games.lock().expect("Idk who you are").get(&url).unwrap().get_players();
    //Check black player
    match b
    {
        Some(p) => (),
        None =>
        {
            b = Some(String::new());
            return format!("You are Black, my condolences.");
        } 
    }

    //Check white player
    match w
    {
        Some(p) => (),
        None =>
        {
            w = Some(String::new());
            return format!("You are White");
        } 
    }
    return String::new();
}