use shoburs::game_pieces;
use crate::game_pieces::*;

#[macro_use] extern crate rocket;

#[get("/api/hello")]
fn hello() -> String 
{
    return format!("Hello, back");
}

#[get("/api/gamestate/default")]
fn get_game_state_default() -> String
{
    let b = Board::new_board(Color::Black, Color::White);
    let state = b.get_state();
    return format!("{:#?}", state);
}

#[launch]
fn rocket() -> _ 
{
    rocket::build().mount("/", routes![hello, get_game_state_default])
}