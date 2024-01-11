use crate::rules::game_rules::*;
use rocket::*;

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

#[get("/api/gamestate/default")]
pub fn get_game_state_default() -> String
{
    let b = Board::new_board(Color::Black, Color::White);
    let state = b.get_state();
    return format!("{:#?}", state);
}

#[get("/create/<id>")]
pub fn create_lobby(id: i8) -> String
{
    return format!("Created lobby {}", id);
}