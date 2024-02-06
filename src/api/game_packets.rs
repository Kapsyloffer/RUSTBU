use serde::{Deserialize, Serialize};

use crate::rules::game_tile::Tile;

use super::move_handling::MovementAction;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub (crate) enum GamePacket {
    Action { //Movement action
        url: String,
        move_p: MovementAction,
        move_a: MovementAction,
    },
    CreateGame, //Call to create new game.

    CheckExists { //Call to check if game exists
        url: String,
    },
    FetchGame { //Call to fetch game state
        url: String,
    },
    FetchMoves { //Call to fetch moves from a rock on a board
        url: String,
        h: Tile,
        c: Tile,
        x: i8,
        y: i8,
        aggr: bool,
    },
    FetchedMoves { //Response containing move positions for rock on the requested board.
        moves: String,
    },
    FetchedGame { //Response containing the requested gamestate.
        state: String,
    },
    GameCreated { //Response containing game url
        url: String,
    },
    JoinGame { 
        url: String, 
        player_id: String,
    }
}