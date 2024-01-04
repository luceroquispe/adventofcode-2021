/*
Grid type for 9x9 matrix coordinate.

TODO: write tests
*/

use crate::common::{Count, PointCount, Coordinate};
use std::fmt;

pub struct Grid {
    cells: Vec<Vec<usize>>,
}

impl Default for Grid {
    fn default() -> Self {
        // empty 9x9 grid
        Self {
            cells: vec![vec![0; 9]; 9],
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Grid:")?;
        for row in &self.cells {
            write!(f, "[")?;
            for (j, cell) in row.iter().enumerate() {
                write!(f, "{}", cell)?;
                if j != row.len() - 1 {
                    write!(f, ", ")?;
                }
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

pub trait GridOperations {
    fn add_point(&mut self, point: Coordinate);
    fn get_count(&self, point: &Coordinate) -> Option<PointCount>;
}

impl GridOperations for Grid {
    fn add_point(&mut self, point: Coordinate) {
        self.cells[point.y][point.x] += 1;
    }

    fn get_count(&self, point: &Coordinate) -> Option<PointCount> {
        Some(PointCount {
            point: *point,
            count: Count(self.cells[point.y][point.x]),
        })
    }
}