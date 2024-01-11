#[macro_use] extern crate rocket;
//Controller functions
use shoburs::api::api_controller::*;

#[launch]
fn rocket() -> _ 
{
    rocket::build()
    .register("/", catchers![not_found, server_error])
    .mount("/", routes![hello, get_game_state_default, create_lobby])
}