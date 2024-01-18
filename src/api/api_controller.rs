use rocket::*;
use crate::rules::game_board::{Color, Board};

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