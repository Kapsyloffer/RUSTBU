#[macro_use] 
extern crate rocket;
use rustbu::api::api_controller::*;
use rustbu::api::api_gamestate::*;
use rustbu::rules::game_state::GameHodler;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(GameHodler::default())
        .mount("/", routes![hello, join_game_instance, serde_test, serde_test2, get_all_games])
        .mount("/api/game/", routes![new_game_instance, get_game_instance])
        .register("/", catchers![not_found, server_error])
}
