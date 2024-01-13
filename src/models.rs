use crate::schema::games;
use diesel::*;

#[derive(Debug, Insertable, Queryable)]
#[table_name = "games"]
pub struct GameInstance
{
    pub game_id: i32,
    pub board_bw: String,
    pub board_bb: String,
    pub board_wb: String,
    pub board_ww: String,
    pub turn: String,
}