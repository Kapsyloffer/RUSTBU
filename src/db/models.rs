use crate::game_state::Game;
use diesel::*;

#[derive(Queryable)]
pub struct GameInstance 
{
    id: i32,
    state: Game
}