#[macro_use] 
extern crate rocket;
use rustbu::api::api_controller::*;
use rustbu::api::api_gamestate::*;
use rustbu::rules::game_state::GameHodler;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(GameHodler::new())
        .mount("/", routes![join_game_instance, set_cookie, get_cookie])
        .mount("/api/game/", routes![new_game_instance, get_game_instance, who_am_i])
        .register("/", catchers![not_found, server_error])
}
