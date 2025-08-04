// src/game.rs

//! The `game` module is responsible for managing the overall game state.
//!
//! It orchestrates the interactions between the game board and the player,
//! tracking whether the game is in progress, has been won, or has been lost.
//! This module will be the primary entry point for the front-end to interact
//! with the game logic.

// The Game struct will hold the game's state.
pub struct Game {
    // The game board. The board module will define the Board struct.
    // board: Board,

    // The current state of the game.
    state: GameState,
}

// GameState represents the possible states of the game.
#[derive(Debug)]
pub enum GameState {
    /// The game is currently in progress.
    InProgress,
    /// The player has won the game.
    Won,
    /// The player has lost the game.
    Lost,
}

impl Game {
    /// Creates a new game.
    ///
    /// # Arguments
    ///
    /// * `dimensions` - A vector defining the size of each dimension of the board.
    /// * `num_mines` - The number of mines to place on the board.
    pub fn new(dimensions: Vec<usize>, num_mines: usize) -> Self {
        // TODO: Initialize the board and place the mines.
        // let board = Board::new(dimensions, num_mines);
        Self {
            // board,
            state: GameState::InProgress,
        }
    }

    /// Returns the current state of the game.
    pub fn state(&self) -> &GameState {
        &self.state
    }
}
