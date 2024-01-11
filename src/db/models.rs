use crate::game_state::Game;
use diesel::*;

#[derive(Queryable)]
pub struct GameInstance 
{
    _id: i32,
    _state: Game
}