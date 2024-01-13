use crate::rules::game_rules::*;
use rocket::*;
//use crate::game_state::Game;
use diesel::prelude;
use diesel::sqlite::*;

#[catch(404)]
pub fn not_found() -> String 
{
    return format!("404");
}

#[catch(500)]
pub fn server_error() -> String
{
    return format!("500: Internal Server Error");
}

#[get("/api/hello")]
pub fn hello() -> String 
{
    return format!("Hello, back");
}

#[post("/api/game/new")]
pub fn new_game_instance() 
{
    let _b = Board::new_board(Color::Black, Color::White);
}

#[post("/api/join/<id>")]
pub fn join_game_instance(id: i32) 
{
    let _b = Board::new_board(Color::Black, Color::White);
}

#[get("/create/<id>")]
pub fn create_lobby(id: i8) -> String
{
    return format!("Created lobby {}", id);
}