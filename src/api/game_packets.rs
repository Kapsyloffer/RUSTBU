use serde::{Deserialize, Serialize};

use super::move_handling::MovementAction;
use crate::rules::game_board::Color;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub(crate) enum GamePacket {
    Action {
        //Movement action
        url: String,
        move_p: MovementAction,
        move_a: MovementAction,
    },
    CreateGame, //Call to create new game.

    CheckExists {
        //Call to check if game exists
        url: String,
    },
    FetchGame {
        //Call to fetch game state
        url: String,
    },
    FetchMoves {
        //Call to fetch moves from a rock on a board
        url: String,
        h: Color,
        c: Color,
        x: i8,
        y: i8,
        aggr: bool,
    },
    FetchedMoves {
        //Response containing move positions for rock on the requested board.
        moves: String,
    },
    FetchedGame {
        //Response containing the requested gamestate.
        state: String,
    },
    GameCreated {
        //Response containing game url
        url: String,
    },
    JoinGame {
        url: String,
        player_id: String,
    },
}
