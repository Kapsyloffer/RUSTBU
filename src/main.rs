#[macro_use] 
extern crate rocket;
use rustbu::api::api_controller::*;
use rustbu::api::api_gamestate::*;
use rustbu::rules::game_hodler::GameHodler;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(GameHodler::new())
        .mount("/", routes![join_game_instance])
        .mount("/api/game/", routes![new_game_instance, get_game_instance, who_am_i, make_move])
        .register("/", catchers![not_found, server_error])
}
