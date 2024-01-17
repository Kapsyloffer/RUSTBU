use rocket::*;
use crate::rules::game_board::{Color, Board};
use crate::rules::game_state::Game;
//use serde_json;

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

use rocket::http::Cookie;
use rocket::http::CookieJar;

#[get("/set")]
pub fn set_cookie(kakburk: &CookieJar) -> String 
{
    let cookie = Cookie::new("my_cookie", Game::generate_url());

    kakburk.add(cookie.clone());

    if kakburk.get("my_cookie").is_none()
    {
        return "ded".into();
    }
    return kakburk.get("my_cookie").unwrap().value().to_string();
}


/*
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
 */