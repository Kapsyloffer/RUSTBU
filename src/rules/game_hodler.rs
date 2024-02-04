use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::sync::broadcast;

use super::game_instance::Game;

#[derive(Debug)]
pub struct Lobby {
    pub game: Game,
    pub sender: broadcast::Sender<()>,
}

#[derive(Debug, Clone)]
pub struct GameHodler {
    pub games: Arc<Mutex<HashMap<String, Lobby>>>,
}

impl GameHodler {
    pub fn new() -> GameHodler {
        return GameHodler {
            games: Arc::new(Mutex::new(HashMap::new())),
        };
    }
}
