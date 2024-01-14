#[macro_use]
extern crate diesel;
#[macro_use] 
extern crate rocket;
use shoburs::api::api_controller::*;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello, join_game_instance, new_game_instance, create_lobby, serde_test, serde_test2])
        .register("/", catchers![not_found, server_error])
}
