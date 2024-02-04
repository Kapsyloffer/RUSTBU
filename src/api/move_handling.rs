use axum::extract::ws::{Message, WebSocket};
use serde::{Deserialize, Serialize};

use crate::{
    api::game_packets::*,
    rules::{game_board::Color, game_hodler::GameHodler, game_tile::Tile},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MovementAction {
    board_colour: Color,
    home_colour: Color,
    x1: i8,
    y1: i8,
    x2: i8,
    y2: i8,
    aggr: bool,
}

pub async fn do_move(
    game_hodler: &GameHodler,
    url: &String,
    move_p: &MovementAction,
    move_a: &MovementAction,
) {
    let mut games = game_hodler.games.lock().unwrap();
    let Some(lobby) = games.get_mut(url) else {
        return;
    };
    if move_p.board_colour == move_a.board_colour {
        return;
        //panic!("Cannot move on the same colour");
    }
    //Make move on p
    let board_p = lobby
        .game
        .get_board_mut(move_p.home_colour, move_p.board_colour)
        .unwrap();
    let b4_p = board_p.clone(); //In case it breaks
    let moved_p: bool = Tile::passive_move(board_p, (move_p.x1, move_p.y1), (move_p.x2, move_p.y2));
    println!("moved_p: {moved_p}");

    //Make move on a
    let board_a = lobby
        .game
        .get_board_mut(move_a.home_colour, move_a.board_colour)
        .unwrap();
    let b4_a = board_a.clone(); //In case it breaks
    let moved_a: bool =
        Tile::aggressive_move(board_a, (move_a.x1, move_a.y1), (move_a.x2, move_a.y2));
    println!("moved_a: {moved_a}");

    //If either move fail.
    if !moved_p || !moved_a {
        //Reset passive move board
        lobby
            .game
            .get_board_mut(move_p.home_colour, move_p.board_colour)
            .unwrap()
            .set_state(b4_p.get_state());

        //Reset aggressive move board
        lobby
            .game
            .get_board_mut(move_a.home_colour, move_a.board_colour)
            .unwrap()
            .set_state(b4_a.get_state());

        //return;
    } else {
        lobby.game.next_turn();
    }

    println!("{}", lobby.game.display());
}

pub async fn fetch_moves(
    socket: &mut WebSocket,
    game_hodler: &GameHodler,
    url: &String,
    h: &Color,
    c: &Color,
    x: &i8,
    y: &i8,
    aggr: &bool,
) {
    let packet = {
        let binding = game_hodler.games.lock().unwrap();
        let b = binding.get(url).unwrap().game.get_board(*h, *c).unwrap();

        let move_list = format!("{:?}", Tile::get_possible_moves(b, *aggr, (*x, *y)));
        println!("fetch_moves: {}", move_list);

        GamePacket::FetchedMoves { moves: move_list }
    };
    if socket
        .send(Message::Text(serde_json::to_string(&packet).unwrap()))
        .await
        .is_err()
    {
        return;
    }
}
