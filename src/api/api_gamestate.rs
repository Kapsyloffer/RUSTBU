use crate::rules::game_board::*;
use rocket::*;
use crate::rules::game_state::{Game, GameHodler};

#[get("/api/game/new")]
pub fn new_game_instance<'r>(shared: &State<GameHodler>) 
{
    #[allow(unused)]
    let g =Game::new_game();
    let mut games_map = shared.games.lock().expect("FAILED TO LOCk");
    let index = games_map.len() as i32;
    games_map.insert(index, g);
    //TODO: Call gamehodler.add_game(g)
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

#[get("/create/<id>")]
pub fn create_lobby(id: i8) -> String
{
    return format!("Created lobby {}", id);
}