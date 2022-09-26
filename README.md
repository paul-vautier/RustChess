# Rust Chess engine

This project was made with the intent of learning Rust. It features a playable chess game and a Chess engine

## File structure 
```
│   main.rs
│   util.rs
│   view.rs
│
├───model
│   │   actions.rs
│   │   algebraic_notation.rs
│   │   board.rs
│   │   mod.rs
│   │   piece.rs
│   │
│   └───chess_actions
│           capture.rs
│           castle.rs
│           mod.rs
│           movement.rs
│           promote.rs
│
├───util
│       util.rs
│
└───view
        console.rs
        tetra_state.rs
```
## Libraries

 - Tetra game engine to control the game flow

 ## Credits 
 Chess set : https://opengameart.org/content/chess-pieces-and-board-squares