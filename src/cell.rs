// src/cell.rs

//! The `cell` module defines the properties of a single cell on the game board.
//!
//! Each cell can be in various states, and can either be a mine or be empty.

// The Cell struct represents a single cell on the board.
pub struct Cell {
    /// The state of the cell.
    state: CellState,

    /// The kind of the cell (mine or empty).
    kind: CellKind,
}

// CellState represents the visibility of a cell.
pub enum CellState {
    /// The cell is hidden from the player.
    Hidden,
    /// The cell has been revealed.
    Revealed,
    /// The cell has been flagged by the player as a potential mine.
    Flagged,
}

// CellKind represents the content of a cell.
pub enum CellKind {
    /// The cell is a mine.
    Mine,
    /// The cell is empty, and contains a count of adjacent mines.
    Empty { adjacent_mines: u8 },
}

impl Cell {
    /// Creates a new, hidden, empty cell.
    pub fn new() -> Self {
        Self {
            state: CellState::Hidden,
            kind: CellKind::Empty { adjacent_mines: 0 },
        }
    }
}
