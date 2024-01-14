use diesel::prelude::*;
use crate::schema::game_instance;

#[derive(Insertable, Debug)]
#[diesel(table_name = game_instance)]  // Corrected table_name attribute
pub struct GameInstance {
    pub id: i32,
    pub game_id: i32,
    pub board_bw: String,
    pub board_bb: String,
    pub board_wb: String,
    pub board_ww: String,
    pub turn: String,
}
