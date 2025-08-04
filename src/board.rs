// src/board.rs

//! The `board` module defines the game board and its properties.
//!
//! The board is an N-dimensional grid of cells. This module is responsible for:
//! - Storing the layout of cells.
//! - Placing mines.
//! - Calculating the number of adjacent mines for each cell.
//! - Handling the logic for revealing cells.

use crate::cell::{Cell, CellKind};
use rand::seq::SliceRandom;

// The Board struct will represent the N-dimensional game board.
pub struct Board {
    /// The dimensions of the board (e.g., `vec![10, 10]` for a 2D 10x10 board).
    dimensions: Vec<usize>,

    /// The cells of the board, stored in a flat vector.
    /// The mapping from N-dimensional coordinates to a 1D index is a key part
    /// of this implementation.
    pub cells: Vec<Cell>,

    /// The total number of mines on the board.
    num_mines: usize,
}

impl Board {
    /// Creates a new board with the given dimensions and number of mines.
    ///
    /// # Arguments
    ///
    /// * `dimensions` - A vector defining the size of each dimension.
    /// * `num_mines` - The number of mines to place.
    pub fn new(dimensions: Vec<usize>, num_mines: usize) -> Self {
        // Calculate the total number of cells.
        let total_cells = dimensions.iter().product();

        // Create the cells.
        let mut cells = vec![Cell::new(); total_cells];

        // Place the mines.
        Self::place_mines(&mut cells, num_mines);

        Self {
            dimensions,
            cells,
            num_mines,
        }
    }

    /// Places mines randomly on the board.
    fn place_mines(cells: &mut Vec<Cell>, num_mines: usize) {
        let mut rng = rand::thread_rng();
        let mine_indices = (0..cells.len()).collect::<Vec<usize>>();
        let chosen_indices = mine_indices.choose_multiple(&mut rng, num_mines);

        for &index in chosen_indices {
            cells[index].kind = CellKind::Mine;
        }
    }
}
