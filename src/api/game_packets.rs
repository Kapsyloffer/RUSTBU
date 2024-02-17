use serde::{Deserialize, Serialize};

use crate::rules::game_tile::Tile;

use super::move_handling::MovementAction;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub (crate) enum GamePacket {
    //Call to make a move.
    Action { 
        url: String,
        move_p: MovementAction,
        move_a: MovementAction,
    },
    //Call to create new game.
    CreateGame {
        player_id: String,
        color: Tile,
    },
    //Call to fetch game state
    FetchGame { 
        url: String,
    },
    //Call to fetch moves from a rock on a given board
    FetchMoves { 
        url: String,
        h: Tile,
        c: Tile,
        x: i8,
        y: i8,
        aggr: bool,
        player: String,
    },
    //Response containing possible move positions for rock on the requested board.
    FetchedMoves { 
        moves: String,
    },
    //Response containing the requested gamestate.
    FetchedGame { 
        state: String,
    },
    //Response containing game url
    GameCreated { 
        url: String,
    },
    //Request to join a game
    JoinGame { 
        url: String, 
        player_id: String,
    },
    //Request previous moves made.
    FetchPreviousMoves{
        url: String
    },
    //Response containing previous moves made.
    PreviousMoves{
        move_p: MovementAction,
        move_a: MovementAction,
    }
}