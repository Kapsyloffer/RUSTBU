//Alterantive name: StupidFish

use crate::{api::move_handling::MovementAction, rules::{game_board::Board, game_instance::Game, game_tile::Tile}};

pub struct ChumBucket {
    best_move_p: Option<MovementAction>,
    best_move_a: Option<MovementAction>,
    best_rock_count: i8,
    best_range: i8 
}

impl ChumBucket {
    pub fn new() -> ChumBucket {
        return ChumBucket{best_move_p: None, best_move_a: None, best_rock_count: 100, best_range: 0};
    }

    //This AI is stupid and evaluates only based on:
    //Freedom of movement (Higher is better)
    //& Enemy rocks remaining. (Lower is better)
    pub fn get_move(&mut self, game: &mut Game, ai_color: Tile) -> (MovementAction, MovementAction) {
        //First we get the opponent colour
        let opp_color = Self::get_opponent(ai_color);

        //Get all boards
        let home_b = *game.get_board(ai_color, Tile::Black).unwrap();
        let home_w = *game.get_board(ai_color, Tile::White).unwrap();
        let opp_b = *game.get_board(opp_color, Tile::Black).unwrap();
        let opp_w = *game.get_board(opp_color, Tile::White).unwrap();

        //Evaluate for each.
        self.eval_move(&home_b, &opp_w,  game, &ai_color);
        self.eval_move(&home_b, &home_w, game, &ai_color);
        self.eval_move(&home_w, &opp_b,  game, &ai_color);
        self.eval_move(&home_w, &home_b, game, &ai_color);

        return (self.best_move_p.clone().unwrap(), self.best_move_a.clone().unwrap());
    }

    pub fn eval_move(&mut self, home_board: &Board, opp_board: &Board, game_state: &Game, ai_color: &Tile) {
        let rock_positions_passive = Self::get_rock_positions(&home_board, *ai_color);
        let rock_positions_aggressive = Self::get_rock_positions(&opp_board, *ai_color);

        //Runs for each rock on home.
        for passive_pos in rock_positions_passive {

            //Get each move for this rock
            let moves = Tile::get_possible_moves(&home_board, false, passive_pos);

            //Get the move deltas
            for m in moves {
                //Deltas
                let dy = m.0 - passive_pos.0;
                let dx = m.1 - passive_pos.1;

                //Try to make moves
                for aggr_pos in &rock_positions_aggressive {
                    //So that we don't mess up the og states.
                    let mut home_clone = home_board.clone();
                    let mut opp_clone = opp_board.clone();
                    
                    //New aggr pos defined using deltas
                    let new_aggr_pos: (i8, i8) = (aggr_pos.0 + dy, aggr_pos.1 + dx);

                    //If both true both moves are valid.
                    let moved_p = Tile::passive_move(&mut home_clone, passive_pos, m);
                    let moved_a = Tile::aggressive_move(&mut opp_clone, *aggr_pos, new_aggr_pos);

                    //Evaluate.
                    if moved_p && moved_a {
                        //Gamestate clone so that we don't mess anything up.
                        let game_clone = game_state.clone();

                        let mut rock_count:i8 = 0;
                        let mut range_count:i8 = 0;

                        //Replace used boards on game_state, then eval
                        for mut game_board in game_clone.get_boards() {
                            //If both home and colour match for home_b
                            if game_board.get_home() == home_clone.get_home() 
                            && game_board.get_color() == home_clone.get_color() {
                                game_board.set_state(home_clone.get_state());
                            }

                            //If both home and colour match for opp_b
                            if game_board.get_home() == opp_clone.get_home() 
                            && game_board.get_color() == opp_clone.get_color() {
                                game_board.set_state(opp_clone.get_state());
                            }

                            let opp_colour = Self::get_opponent(*ai_color);
                            //Eval range
                            range_count += Self::get_rock_positions(&game_board, *ai_color).len() as i8;

                            //Eval Opponent rocks
                            rock_count += Self::get_rock_positions(&game_board, opp_colour).len() as i8;
                        }
                    
                        if rock_count <= self.best_rock_count /*&& range_count > self.best_range*/ {
                            //Obv very good
                            self.best_rock_count = rock_count;
                            self.best_range = range_count;

                            let move_p = MovementAction::new(home_clone.get_home(),
                                home_clone.get_color(),
                                passive_pos.1,
                                passive_pos.0,
                                m.1,
                                m.0,
                                false,
                                String::from("ChumBucketAI")
                            );
                            let move_a = MovementAction::new(opp_clone.get_home(),
                                opp_clone.get_color(),
                                aggr_pos.1,
                                aggr_pos.0,
                                new_aggr_pos.1,
                                new_aggr_pos.0,
                                true,
                                String::from("ChumBucketAI")
                            );

                            println!("\nOUR BEST MOVE:\nROCKS:{}\nRANGE:{}\nMOVE_P{:#?}\nMOVE_A:{:#?}", 
                            rock_count, range_count, move_p, move_a);

                            self.best_move_p = Some(move_p);
                            self.best_move_a = Some(move_a);
                        }
                    }
                }
            }
        }
    }

    pub fn get_rock_positions(b: &Board, target: Tile) -> Vec<(i8, i8)> {
        let board_state = b.get_state();
        let mut rock_positions: Vec<(i8, i8)> = Vec::new();
        //Go through each tile in the board and see if it's our rock coloures.
        for x in 0..=3 {
            for y in 0..=3 {
                if board_state[y][x] == target {
                    rock_positions.push((y as i8, x as i8));
                }
            }
        }
        return rock_positions;
    }

    fn get_opponent(color: Tile) -> Tile {
        match color {
            Tile::Black => return Tile::White,
            Tile::White => return Tile::Black,
            Tile::Empty => unimplemented!(),
        }
    }
}