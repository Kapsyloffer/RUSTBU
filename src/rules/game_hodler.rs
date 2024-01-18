use std::sync::{Mutex, Arc};
use std::collections::HashMap;

use super::game_instance::Game;


#[derive(Debug)]
pub struct GameHodler
{
   pub games: Arc<Mutex<HashMap<String, Game>>>
}

impl GameHodler
{
    pub fn new () -> GameHodler
    {
        return GameHodler{games: Arc::new(Mutex::new(HashMap::new()))}
    }
}