// src/coordinates.rs

//! The `coordinates` module provides utilities for working with N-dimensional coordinates.
//!
//! This is a critical part of the N-dimensional Minesweeper engine. It handles
//! the conversion between N-dimensional coordinates and a 1D index into a flat
//! vector, which is how the board's cells are stored. It also provides a way
//! to iterate over the neighbors of a cell in N-dimensional space.

/// A type alias for N-dimensional coordinates.
pub type Coordinates = Vec<usize>;

/// Converts N-dimensional coordinates to a 1D index.
///
/// # Arguments
///
/// * `coords` - The N-dimensional coordinates.
/// * `dimensions` - The dimensions of the N-dimensional grid.
pub fn to_index(coords: &Coordinates, dimensions: &[usize]) -> usize {
    // This is a classic row-major order mapping.
    // For example, in 2D (row, col) with dimensions (width, height),
    // the index is `row * width + col`.
    // In 3D (x, y, z) with dimensions (width, height, depth),
    // the index is `z * (width * height) + y * width + x`.
    // This generalizes to N dimensions.
    let mut index = 0;
    let mut stride = 1;
    for (i, &coord) in coords.iter().enumerate() {
        if i > 0 {
            stride *= dimensions[i - 1];
        }
        index += coord * stride;
    }
    index
}

/// Converts a 1D index to N-dimensional coordinates.
///
/// # Arguments
///
/// * `index` - The 1D index.
/// * `dimensions` - The dimensions of the N-dimensional grid.
pub fn to_coords(mut index: usize, dimensions: &[usize]) -> Coordinates {
    let mut coords = vec![0; dimensions.len()];
    let mut stride = 1;
    for (i, &dim) in dimensions.iter().enumerate() {
         if i > 0 {
            stride *= dimensions[i-1];
        }
    }

    for (i, &dim) in dimensions.iter().enumerate().rev() {
        coords[i] = index / stride;
        index %= stride;
        if i > 0 {
            stride /= dimensions[i-1];
        }
    }
    coords
}

// TODO: Add a function to get the neighbors of a cell in N-dimensional space.
// This will involve iterating through all combinations of (-1, 0, 1) for each dimension.
