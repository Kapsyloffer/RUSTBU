use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::api::move_handling::MovementAction;
use super::game_instance::Game;

#[derive(Debug, Clone)]
pub struct GameHodler {
    pub games: Arc<Mutex<HashMap<String, Game>>>,
    pub moves: Arc<Mutex<HashMap<String, (MovementAction, MovementAction)>>>,
}

impl GameHodler {
    pub fn new() -> GameHodler {
        return GameHodler {
            games: Arc::new(Mutex::new(HashMap::new())),
            moves: Arc::new(Mutex::new(HashMap::new())),
        };
    }
}
