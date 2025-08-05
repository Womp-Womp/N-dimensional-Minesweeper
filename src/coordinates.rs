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
    for (i, &_dim) in dimensions.iter().enumerate() {
         if i > 0 {
            stride *= dimensions[i-1];
        }
    }

    for (i, &_dim) in dimensions.iter().enumerate().rev() {
        coords[i] = index / stride;
        index %= stride;
        if i > 0 {
            stride /= dimensions[i-1];
        }
    }
    coords
}

/// Returns a list of valid neighbor coordinates for a given cell.
///
/// This function explores all adjacent cells in an N-dimensional grid. An adjacent
/// cell is one that can be reached by changing each coordinate by -1, 0, or 1.
/// The function excludes the cell's own coordinates from the result.
///
/// # Arguments
///
/// * `coords` - The N-dimensional coordinates of the cell.
/// * `dimensions` - The dimensions of the board.
///
/// # Returns
///
/// A `Vec<Coordinates>` containing the coordinates of all valid neighbors.
pub fn get_neighbors(coords: &Coordinates, dimensions: &[usize]) -> Vec<Coordinates> {
    let mut neighbors = Vec::new();
    let num_dimensions = coords.len();
    if num_dimensions == 0 {
        return neighbors;
    }

    let num_neighbors_to_check = 3_u32.pow(num_dimensions as u32);
    let center_index = (num_neighbors_to_check - 1) / 2;

    'outer: for i in 0..num_neighbors_to_check {
        if i == center_index {
            continue;
        }

        let mut temp_coords = coords.clone();
        let mut n = i;

        for j in 0..num_dimensions {
            let offset = (n % 3) as i32 - 1;
            n /= 3;

            // Check for underflow before applying the offset
            if offset == -1 && temp_coords[j] == 0 {
                continue 'outer;
            }

            let new_coord = (temp_coords[j] as i32 + offset) as usize;

            // Check for overflow
            if new_coord >= dimensions[j] {
                continue 'outer;
            }

            temp_coords[j] = new_coord;
        }

        neighbors.push(temp_coords);
    }

    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_neighbors_2d_center() {
        let dimensions = vec![3, 3];
        let coords = vec![1, 1];
        let mut neighbors = get_neighbors(&coords, &dimensions);
        neighbors.sort(); // Sort for consistent comparison

        let mut expected = vec![
            vec![0, 0], vec![0, 1], vec![0, 2],
            vec![1, 0],             vec![1, 2],
            vec![2, 0], vec![2, 1], vec![2, 2],
        ];
        expected.sort();

        assert_eq!(neighbors, expected);
    }

    #[test]
    fn test_get_neighbors_2d_corner() {
        let dimensions = vec![3, 3];
        let coords = vec![0, 0];
        let mut neighbors = get_neighbors(&coords, &dimensions);
        neighbors.sort();
        let mut expected = vec![vec![0, 1], vec![1, 0], vec![1, 1]];
        expected.sort();
        assert_eq!(neighbors, expected);
    }

    #[test]
    fn test_get_neighbors_2d_edge() {
        let dimensions = vec![3, 3];
        let coords = vec![0, 1];
        let mut neighbors = get_neighbors(&coords, &dimensions);
        neighbors.sort();
        let mut expected = vec![
            vec![0, 0], vec![0, 2],
            vec![1, 0], vec![1, 1], vec![1, 2],
        ];
        expected.sort();
        assert_eq!(neighbors, expected);
    }

    #[test]
    fn test_get_neighbors_1d() {
        let dimensions = vec![3];
        let coords = vec![1];
        let mut neighbors = get_neighbors(&coords, &dimensions);
        neighbors.sort();
        let mut expected = vec![vec![0], vec![2]];
        expected.sort();
        assert_eq!(neighbors, expected);
    }

    #[test]
    fn test_get_neighbors_3d_center() {
        let dimensions = vec![3, 3, 3];
        let coords = vec![1, 1, 1];
        let neighbors = get_neighbors(&coords, &dimensions);
        assert_eq!(neighbors.len(), 26);
    }
}
