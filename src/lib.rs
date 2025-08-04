// N-dimensional Minesweeper: The Library
//
// This library contains the core game logic for the N-dimensional Minesweeper game.
// By separating the game logic into a library, we can easily create different
// front-ends (e.g., a command-line interface, a graphical user interface)
// that all use the same underlying game engine.

// The library is composed of several modules, each with a specific responsibility.
// This modular design enhances maintainability and makes the codebase easier to understand.

// Declare the modules that make up the library.
pub mod board;
pub mod cell;
pub mod coordinates;
pub mod game;

// The `prelude` module is a common pattern in Rust libraries.
// It re-exports the most commonly used items for convenience.
pub mod prelude {
    pub use crate::board::Board;
    pub use crate::cell::{Cell, CellKind, CellState};
    pub use crate::coordinates::{to_coords, to_index, Coordinates};
    pub use crate::game::{Game, GameState};
}
