# Rust Chess engine

This project was made with the intent of learning Rust. 

It features a playable chess game and a Chess engine. The engine current goal initially was not about performances (i.e. using [bitboards](https://www.chessprogramming.org/Bitboards)), but about learning how to right an idiomatic rust application.

## File structure 
```
│   main.rs
│
├───model                               // The model of the chess game
│   │   actions.rs                      // Possible actions generationg
│   │   algebraic_notation.rs           // Algebraic notation parsing
│   │   board.rs                        // Chess board (8x8 with 10x12 wrapper)
│   │   mod.rs                      
│   │   piece.rs                        // Chess enumeration with possible moves
│   │   
│   └───chess_actions                   // Chess actions, command pattern implementation (do / undo)
│           capture.rs
│           castle.rs
│           mod.rs
│           movement.rs
│           promote.rs
│
├───util
│       mod.rs
│       util.rs
│
└───view                                // Related to the view
        console.rs                      // Terminal view
        mod.rs
        tetra_state.rs                  // GUI with Tetra
```
## Libraries

 - Tetra game engine to control the game flow

 ## Credits 
 - Chess set : https://opengameart.org/content/chess-pieces-and-board-squares
 - Chess programming notions : https://www.chessprogramming.org/Main_Page
