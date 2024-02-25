use axum::extract::ws::{Message, WebSocket};
use serde::{Deserialize, Serialize};

use crate::{api::game_packets::*, rules::{game_board::Board, game_hodler::GameHodler, game_tile::Tile}};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MovementAction {
    board_colour: Tile,
    home_colour: Tile,
    x1: i8,
    y1: i8,
    x2: i8,
    y2: i8,
    aggr: bool,
    player: String,
}

pub async fn do_move(game_hodler: &GameHodler, url: &String, move_p: &MovementAction, move_a: &MovementAction) {
    let mut games = game_hodler.games.lock().unwrap();
    let Some(game) = games.get_mut(url) else {
        return;
    };
    let turn = game.get_turn();

    //You may not move if the game is ended.
    if game.has_winner(){
        return;
    }

    //You may not move on a same coloured board.
    if move_p.board_colour == move_a.board_colour {
        return;
    }

    //In case the passive and aggressive move differ in size and direction.
    if move_p.x1 - move_p.x2 != move_a.x1 - move_a.x2
    || move_p.y1 - move_p.y2 != move_a.y1 - move_a.y2
    {
        return;
    }

    //In case the passive move is not made on your homeboard.
    if move_p.home_colour != turn{
        return;
    }

    //Make a move on p
    let board_p = game
        .get_board(move_p.home_colour, move_p.board_colour)
        .unwrap();
    //In case it breaks
    let b4_p = board_p.clone(); 

    //You may not move if it's not your turn.
    if b4_p.get_state()[move_p.x1 as usize][move_p.y1 as usize] != turn{
        return;
    }

    let moved_p: bool = Tile::passive_move(board_p, (move_p.x1, move_p.y1), (move_p.x2, move_p.y2));

    //Make a move on a
    let board_a = game
        .get_board(move_a.home_colour, move_a.board_colour)
        .unwrap();
    //In case it breaks
    let b4_a = board_a.clone(); 

    //You may not move if it's not your turn.
    if b4_a.get_state()[move_a.x1 as usize][move_a.y1 as usize] != turn{
        return;
    }

    let moved_a: bool = Tile::aggressive_move(board_a, (move_a.x1, move_a.y1), (move_a.x2, move_a.y2));

    //If either move fail we reset the board states.
    if !moved_p || !moved_a {
        //Reset passive move board
        game.get_board(move_p.home_colour, move_p.board_colour)
            .unwrap()
            .set_state(b4_p.get_state());

        //Reset aggressive move board
        game.get_board(move_a.home_colour, move_a.board_colour)
            .unwrap()
            .set_state(b4_a.get_state());
    } else {
        let winner = Board::check_winner(&board_a);
        game.set_winner(&winner);
        
        if winner != Tile::Empty {
            println!("Winner for game {}: {:?}", url, winner);
        }
        //Insert previous move in the game hodler.
        game_hodler.moves.lock().unwrap().insert(String::from(url), (move_p.clone(), move_a.clone())); 
        game.next_turn();
    }

    println!("{}", game.display());
}

pub async fn fetch_moves(socket: &mut WebSocket, game_hodler: &GameHodler, url: &String, h: &Tile, c: &Tile, x: &i8, y: &i8, aggr: &bool, player: &String) {
    let mut binding = game_hodler.games.lock().unwrap().to_owned();
    let b = binding.get_mut(url).unwrap().get_board(*h, *c).unwrap();
    //This is so stupid.
    let binding2 =  game_hodler.games.lock().unwrap().to_owned();
    let game = binding2.get(url).unwrap();

    let mut move_list = format!("{:?}", Tile::get_possible_moves(b, *aggr, (*x, *y)));

    //Cannot fetch if it's not your turn.
    if game.is_player(player) != game.get_turn() {
        move_list = format!("[]");
    }

    //Cannot fetch if it's not your piece
    if b.get_state()[*x as usize][*y as usize] != game.is_player(player) {
        move_list = format!("[]");
    }

    //Cannot make a passive move outside of your own homeboards.
    if !aggr && game.is_player(player) != b.get_home() {
        move_list = format!("[]");
    }

    //Cannot fetch moves if the game is over.
    if game.has_winner() {
        move_list = format!("[]");
    }

    //Cannot fetch moves if the game has not started.
    if game.get_players().0 == "None" || game.get_players().1 == "None" {
        move_list = format!("[]");
    }

    //Send it
    let packet = GamePacket::FetchedMoves { moves: move_list };
    if socket
        .send(Message::Text(serde_json::to_string(&packet).unwrap()))
        .await
        .is_err() {
        return;
    }
}
