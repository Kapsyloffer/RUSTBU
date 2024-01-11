use shoburs::game_rules;
use crate::game_rules::*;

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

#[get("/create/<id>")]
fn create_lobby(id: i8) -> String
{
    return format!("Created lobby {}", id);
}

#[launch]
fn rocket() -> _ 
{
    rocket::build().mount("/", routes![hello, get_game_state_default, create_lobby])
}