#[macro_use] extern crate rocket;
use shoburs::api::api_controller::*;
use std::net::Ipv4Addr;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello, get_game_state_default, create_lobby])
        .register("/", catchers![not_found, server_error])
}
