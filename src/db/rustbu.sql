CREATE TABLE game_instance (
    id SERIAL PRIMARY KEY,
    game_id INTEGER NOT NULL,
    game_state TEXT NOT NULL
);