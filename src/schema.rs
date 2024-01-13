use diesel::*;

table! {
    games {
        id -> Integer,
        game_id -> Integer,
        board_bw -> Text,
        board_bb -> Text,
        board_wb -> Text,
        board_ww -> Text,
        turn -> Text,
    }
}