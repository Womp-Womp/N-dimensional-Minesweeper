// N-dimensional Minesweeper: The Executable
//
// This is the entry point for the N-dimensional Minesweeper game.
// Its primary responsibility is to handle user input, display the game state,
// and call the appropriate functions from the game library.

// We use the `prelude` to conveniently import the most common items from the library.
use n_dimensional_minesweeper::prelude::*;

fn main() {
    println!("Starting N-dimensional Minesweeper!");

    // Create a new 2D game board, 10x10 with 10 mines.
    let dimensions = vec![10, 10];
    let num_mines = 10;
    let game = Game::new(dimensions, num_mines);

    println!("Game created. Current state: {:?}", game.state());

    // In the future, this is where the main game loop will go.
    // It will handle user input, update the game state, and render the board.
}

// We need to add a `Debug` derive to `GameState` to print it.
// I'll do that in `src/game.rs`.
