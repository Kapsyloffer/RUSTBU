#[macro_use] 
extern crate rocket;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rustbu::api::api_controller::*;
use rustbu::api::api_gamestate::*;
use rustbu::rules::game_state::GameHodler;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(GameHodler::default())
        .mount("/", routes![hello, join_game_instance, new_game_instance, create_lobby, serde_test, serde_test2, get_all_games])
        .register("/", catchers![not_found, server_error])
}
