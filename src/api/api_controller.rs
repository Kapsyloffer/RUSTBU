use rocket::*;
use crate::rules::game_state::Game;
use serde_json;

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