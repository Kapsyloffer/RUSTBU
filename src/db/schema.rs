diesel::table! {
    game_instance {
        id -> Integer,
        game_id -> Integer,
        game_state -> Text
    }
}