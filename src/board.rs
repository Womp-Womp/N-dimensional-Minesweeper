// src/board.rs

//! The `board` module defines the game board and its properties.
//!
//! The board is an N-dimensional grid of cells. This module is responsible for:
//! - Storing the layout of cells.
//! - Placing mines.
//! - Calculating the number of adjacent mines for each cell.
//! - Handling the logic for revealing cells.

use crate::cell::{Cell, CellKind, CellState};
use crate::coordinates::{get_neighbors, to_coords, to_index};
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

        let mut board = Self {
            dimensions,
            cells,
            num_mines,
        };

        board.calculate_adjacent_mines();
        board
    }

    /// Calculates and sets the number of adjacent mines for each empty cell.
    fn calculate_adjacent_mines(&mut self) {
        for i in 0..self.cells.len() {
            // We only need to calculate for empty cells
            if self.cells[i].kind == CellKind::Mine {
                continue;
            }

            let coords = to_coords(i, &self.dimensions);
            let neighbors = get_neighbors(&coords, &self.dimensions);

            let mut mine_count = 0;
            for neighbor_coords in neighbors {
                let neighbor_index = to_index(&neighbor_coords, &self.dimensions);
                if self.cells[neighbor_index].kind == CellKind::Mine {
                    mine_count += 1;
                }
            }

            // Update the cell's kind with the mine count
            if let CellKind::Empty { adjacent_mines } = &mut self.cells[i].kind {
                *adjacent_mines = mine_count;
            }
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

    /// Toggles a flag on a cell.
    ///
    /// # Arguments
    ///
    /// * `coords` - The coordinates of the cell to toggle the flag on.
    pub fn toggle_flag(&mut self, coords: &crate::coordinates::Coordinates) {
        let index = to_index(coords, &self.dimensions);
        if let Some(cell) = self.cells.get_mut(index) {
            match cell.state {
                CellState::Hidden => cell.state = CellState::Flagged,
                CellState::Flagged => cell.state = CellState::Hidden,
                CellState::Revealed => (),
            }
        }
    }

    /// Reveals a cell.
    ///
    /// # Arguments
    ///
    /// * `coords` - The coordinates of the cell to reveal.
    ///
    /// # Returns
    ///
    /// * `true` if a mine was revealed, `false` otherwise.
    pub fn reveal(&mut self, coords: &crate::coordinates::Coordinates) -> bool {
        let index = to_index(coords, &self.dimensions);

        // Can't reveal a flagged or already revealed cell
        if self.cells[index].state == CellState::Flagged
            || self.cells[index].state == CellState::Revealed
        {
            return false;
        }

        self.cells[index].state = CellState::Revealed;

        match self.cells[index].kind {
            CellKind::Mine => true,
            CellKind::Empty { adjacent_mines } => {
                if adjacent_mines == 0 {
                    // If the cell is empty and has no adjacent mines, reveal all its neighbors
                    let neighbors = get_neighbors(coords, &self.dimensions);
                    for neighbor_coords in neighbors {
                        self.reveal(&neighbor_coords);
                    }
                }
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::{Cell, CellKind, CellState};

    #[test]
    fn test_calculate_adjacent_mines_2d() {
        let dimensions = vec![3, 3];
        let total_cells = 9;
        let mut cells = vec![Cell::new(); total_cells];

        // Place mines at (0,0) [index 0] and (2,2) [index 8]
        cells[0].kind = CellKind::Mine;
        cells[8].kind = CellKind::Mine;

        let mut board = Board {
            dimensions,
            cells,
            num_mines: 2,
        };

        board.calculate_adjacent_mines();

        // Check adjacent mine counts for a few cells
        // Cell (1,0) [index 1] should have 1 neighbor mine.
        if let CellKind::Empty { adjacent_mines } = board.cells[1].kind {
            assert_eq!(adjacent_mines, 1);
        } else {
            panic!("Cell (1,0) should be empty");
        }

        // Cell (0,1) [index 3] should have 1 neighbor mine.
        if let CellKind::Empty { adjacent_mines } = board.cells[3].kind {
            assert_eq!(adjacent_mines, 1);
        } else {
            panic!("Cell (0,1) should be empty");
        }

        // Cell (1,1) [index 4] should have 2 neighbor mines.
        if let CellKind::Empty { adjacent_mines } = board.cells[4].kind {
            assert_eq!(adjacent_mines, 2);
        } else {
            panic!("Cell (1,1) should be empty");
        }

        // Ensure mine cells are untouched
        assert_eq!(board.cells[0].kind, CellKind::Mine);
        assert_eq!(board.cells[8].kind, CellKind::Mine);
    }

    #[test]
    fn test_toggle_flag() {
        let mut board = Board::new(vec![2, 2], 0);
        let coords = vec![0, 0];

        // Initially hidden
        assert_eq!(board.cells[0].state, CellState::Hidden);

        // Toggle to flagged
        board.toggle_flag(&coords);
        assert_eq!(board.cells[0].state, CellState::Flagged);

        // Toggle back to hidden
        board.toggle_flag(&coords);
        assert_eq!(board.cells[0].state, CellState::Hidden);
    }

    #[test]
    fn test_reveal_mine() {
        let mut board = Board::new(vec![2, 2], 1);
        // Find the mine
        let mine_index = board
            .cells
            .iter()
            .position(|c| c.kind == CellKind::Mine)
            .unwrap();
        let mine_coords = to_coords(mine_index, &board.dimensions);

        // Reveal the mine
        let is_mine = board.reveal(&mine_coords);
        assert!(is_mine);
        assert_eq!(board.cells[mine_index].state, CellState::Revealed);
    }

    #[test]
    fn test_reveal_empty_cell() {
        let mut board = Board::new(vec![3, 3], 0);
        board.cells[0].kind = CellKind::Mine; // Place a mine at (0,0)
        board.calculate_adjacent_mines();
        let coords = vec![1, 1]; // A cell with 1 adjacent mine

        // Reveal the cell
        let is_mine = board.reveal(&coords);
        assert!(!is_mine);
        let index = to_index(&coords, &board.dimensions);
        assert_eq!(board.cells[index].state, CellState::Revealed);
    }

    #[test]
    fn test_flood_fill_reveal() {
        let mut board = Board::new(vec![3, 3], 0);
        board.cells[0].kind = CellKind::Mine; // Mine at (0,0)
        board.calculate_adjacent_mines();

        // Reveal a cell with 0 adjacent mines
        let coords = vec![2, 2];
        board.reveal(&coords);

        // All cells except the mine at (0,0) should be revealed.
        // The mine is at index 0. All others should be revealed.
        for (i, cell) in board.cells.iter().enumerate() {
            if i == 0 {
                // The mine should not be revealed
                assert_ne!(cell.state, CellState::Revealed);
            } else {
                // All other cells should be revealed
                assert_eq!(
                    cell.state,
                    CellState::Revealed,
                    "Cell at index {} was not revealed",
                    i
                );
            }
        }
    }
}
