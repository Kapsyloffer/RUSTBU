#![allow(unused)]
use rocket::{*, http::Cookie};
use crate::rules::game_state::{Game, GameHodler};

//Skapa en ny game lobby med en random url
//t.ex. rsIa8ZVuA
//Och redirectar direkt till den.
#[get("/new")]
pub fn new_game_instance<'r>(shared: &State<GameHodler>) -> response::Redirect
{
    let g = Game::new_game();
    let mut games_map = shared.games.lock().expect("FAILED TO LOCk");

    let url = Game::generate_url();
    games_map.insert(url.to_owned(), g);

    return response::Redirect::to(format!("/api/game/{}", url));
}

//Fetcha en gamestate med associated URL
#[get("/state/<url>")]
pub fn get_game_instance<'r>(url: String, shared: &State<GameHodler>) -> String
{
    let games_map = shared.games.lock().expect("FAILED TO LOCk");
    let g: &Game = games_map.get(&url).unwrap();
    return format!("{:#?}", g);
}

#[post("/<url>/board/<colors>/aggr/<aggr>/move/<y1>/<x1>/<y2>/<x2>")]
fn make_move(url: String, colors: String, aggr: bool, y1: i8, x1: i8, y2: i8, x2: i8, shared: &State<GameHodler>)
{
    let mut board = shared.games.lock().expect("FAILED TO LOCk").get(&url).unwrap().get_boards();
    todo!();
}

//Kolla vilken spelare du är i ett game, om rollen är tom, blir du den rollen.
//I guess used for debugging
#[get("/whoami/<url>")]
pub fn who_am_i(url: String, shared: &State<GameHodler>) -> String
{
    let (mut b, mut w) = shared.games.lock().expect("Idk who you are").get(&url).unwrap().get_players();
    //Check black player
    match b
    {
        Some(p) => (),
        None =>
        {
            b = Some(String::new());
            return format!("You are Black, my condolences.");
        } 
    }

    //Check white player
    match w
    {
        Some(p) => (),
        None =>
        {
            w = Some(String::new());
            return format!("You are White");
        } 
    }
    return String::new();
}