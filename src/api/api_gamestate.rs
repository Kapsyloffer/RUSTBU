use crate::rules::game_board::*;
use rocket::*;
use crate::rules::game_state::{Game, GameHodler};

//Skapa en ny game lobby med en random url
//t.ex. rsIa8ZVuA
#[get("/new")]
pub fn new_game_instance<'r>(shared: &State<GameHodler>) -> String
{
    let g =Game::new_game();
    let mut games_map = shared.games.lock().expect("FAILED TO LOCk");
    let url = Game::generate_url();
    games_map.insert(url.to_owned(), g);
    return url;
}

//Fetcha en gamestate med associated URL
#[get("/<url>")]
pub fn get_game_instance<'r>(url: String, shared: &State<GameHodler>) -> String
{
    let games_map = shared.games.lock().expect("FAILED TO LOCk");
    let g: &Game = games_map.get(&url).unwrap();
    return format!("{:#?}", g);
}


#[get("/get_games")]
pub fn get_all_games(shared: &State<GameHodler>) -> String
{
    let size = shared.games.lock().expect("a").len();
    return format!("Number of games {}", size);
}

#[post("/api/join/<id>")]
pub fn join_game_instance(id: i32) 
{
    let _b = Board::new_board(Color::Black, Color::White);
    println!("{}", id);
}