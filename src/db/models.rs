use diesel::prelude::*;
use crate::db::schema::game_instance;

#[derive(Insertable, Debug)]
#[diesel(table_name = game_instance)]  // Corrected table_name attribute
pub struct GameInstance {
    pub id: i32,
    pub game_id: i32,
    pub game_state: String //Because we now serialize the Game object
}
