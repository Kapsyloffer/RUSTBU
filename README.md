## RUSTBU

![Logo](https://raw.githubusercontent.com/Kapsyloffer/RUSTBU/master/assets/RUSTBU.png)

A lichess clone of SHOBU powered by Rust.

how 2 run:

1. Get rust

2. `cargo run`

3. ???

4. Profit.

Keep in mind that this is just the backend you also need the frontend in order to run it:
https://github.com/Kapsyloffer/RUSTBU-FE

---

## TODO LIST:

### Frontend todo:

- [x] Render the boards on the frontend.
- [x] Create a new game and get a sharable link to it
- [x] Entering a link joins the game.
- [x] Allow moves only if it's your turn.
- [ ] Write good UI.
- [x] Determine which colour a player is.
- [x] A Black player cannot move white rocks, and vice versa.
- [x] Mirror the game for the white player. (i.e. white rocks on bottom)
- [x] Render actual rock positions.
- [ ] Proper use of websockets/broadcasting.
- [x] Move using clicks
- [ ] Move by dragging
- [ ] Render rocks on top of the board, independently, rather than in divs. 
- [ ] Highlight boards where movement is possible. (homeboards on passive, boards of opposite colour on aggresssive)
- [x] Locally change board layouts between standard and "dork" (See figure 1)
- [x] Show possible movement path, like in lichess.
- [x] Max 2 steps per move.
- [x] Take back passive move locally.
- [x] Render local rock positions. i.e. if we moved passive, show it.
- [x] Aggressive move should have the same size and direction as passive.
- [x] Prevent aggressive move on a board of the same colour as the passive move.
- [x] Prevent aggressive move on a board of the same colour as the passive move.
- [x] End the game when we have a winner.
- [ ] Rewrite websocket related functions to support broadcasting.

Figure 1:

```
Standard layout:       "DORK" layout:
    [B][W]                [B][W]
    ------                ------
    [B][W]                [W][B]
```

### Backend todo:

- [x] Passive Movement Phase.
- [x] Prevent pushing any rocks in the passive move.
- [x] Max 2 steps per move.
- [x] Prevent the passive move from being on the opponent's homeboard.
- [x] Prevent pushing any rocks in the passive move.
- [x] Aggressive Movement Phase.
- [x] Prevent pushing of same coloured rock.
- [x] Prevent pushing of more than 2 rocks at once.
- [x] Prevent aggressive move on a board of the same colour as the passive move.
- [x] Prevent aggressive move that's different from the passive move.
- [x] Prevent moves on boards of same colour.
- [x] Check for winner.
- [x] Change active player if both the passive and aggressive move are OK.
- [x] Unit test movement rules.
- [x] Handle websocket connection.
- [x] Verify which player is making a move. Black should not move white rocks.
- [x] Generate link on new game, and redirect.
- [ ] Broadcasting.
- [x] Update state for both players on board change. (TODO: Do properly...)
- [ ] Unit test websocket gameplay.
- [x] Joining a game; lobby owner is black, next person to join is white.
- [x] If a game is full you're spectator. (ish)
- [x] Prevent the first move from being on the opponent's homeboard.
- [x] No moves can be made until the game has started.
- [x] End the game when we have a winner.
- [ ] Prevent duplicate URLs. (Rare issue, but still bad if it happens.)


### Future improvements:

- [ ] Sound effects, like in Lichess/Chess.com
- [ ] Ability to draw arrows like in Lichess/Chess.com
- [ ] Lobby browser.
- [ ] Lobby/in-game chat.
- [x] Option to select your color on lobby creation.
- [ ] Option to surrender.
- [ ] Option to offer draw.
- [ ] Option to ask for takeback.
- [ ] Movement History (+ Algebraic notation).
- [ ] Visual setting, checkered board or "normal" board.
- [ ] bo3, winner plays white next game (goes second).
- [ ] gamemodes?
- [ ] Return to lobby after the game is over.
- [ ] Account.
- [ ] ELO.
- [ ] Friends.
- [ ] Show a little flag next to name.
- [ ] Navigatable page.
- [ ] Themes for boards and pieces.
- [ ] SHOBU engine/AI.

Disclaimer: I do not own SHOBU in any way, shape, or form.  I just find the game neat hence I'm doing this project.

