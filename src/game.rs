// src/game.rs

//! The `game` module is responsible for managing the overall game state.
//!
//! It orchestrates the interactions between the game board and the player,
//! tracking whether the game is in progress, has been won, or has been lost.
//! This module will be the primary entry point for the front-end to interact
//! with the game logic.

use crate::board::Board;
use crate::coordinates::Coordinates;

// The Game struct will hold the game's state.
pub struct Game {
    // The game board. The board module will define the Board struct.
    board: Board,

    // The current state of the game.
    state: GameState,
}

// GameState represents the possible states of the game.
#[derive(Debug, PartialEq)]
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
        let board = Board::new(dimensions, num_mines);
        Self {
            board,
            state: GameState::InProgress,
        }
    }

    /// Returns the current state of the game.
    pub fn state(&self) -> &GameState {
        &self.state
    }

    /// Toggles a flag on a cell.
    pub fn toggle_flag(&mut self, coords: &Coordinates) {
        if self.state == GameState::InProgress {
            self.board.toggle_flag(coords);
        }
    }

    /// Reveals a cell.
    pub fn reveal(&mut self, coords: &Coordinates) {
        if self.state == GameState::InProgress {
            if self.board.reveal(coords) {
                self.state = GameState::Lost;
            } else if self.is_won() {
                self.state = GameState::Won;
            }
        }
    }

    /// Checks if the game has been won.
    fn is_won(&self) -> bool {
        // The game is won if all non-mine cells are revealed.
        self.board
            .cells
            .iter()
            .all(|cell| (cell.kind != crate::cell::CellKind::Mine) == (cell.state == crate::cell::CellState::Revealed))
    }
}
