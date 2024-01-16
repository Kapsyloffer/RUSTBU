use rocket::*;
use crate::rules::{game_state::Game, game_board::{Color, Board}};
use serde_json;

#[catch(404)]
pub fn not_found() -> String 
{
    return format!("404: Not found.");
}

#[catch(500)]
pub fn server_error() -> String
{
    return format!("500: Internal Server Error.");
}

#[post("/join/<id>")]
pub fn join_game_instance(id: i32) 
{
    let _b = Board::new_board(Color::Black, Color::White);
    println!("{}", id);
}

#[get("/api/serialized_game")]
pub fn serde_test() -> String 
{
    let g = Game::new_game();
    let serialized = serde_json::to_string(&g).unwrap();
    return format!("{}", serialized);
}

#[get("/api/unserialized_game/<serialized>")]
pub fn serde_test2(serialized: &str) -> String 
{
    let mut dserialized: Game = serde_json::from_str(&serialized).unwrap();
    dserialized.next_turn();
    return format!("{:#?}", dserialized);
}