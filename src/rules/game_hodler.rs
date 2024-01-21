use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::game_instance::Game;

#[derive(Debug, Clone)]
pub struct GameHodler {
    pub games: Arc<Mutex<HashMap<String, Game>>>,
}

impl GameHodler {
    pub fn new() -> GameHodler {
        return GameHodler {
            games: Arc::new(Mutex::new(HashMap::new())),
        };
    }
}
