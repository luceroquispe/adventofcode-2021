/*
Grid type for 9x9 matrix coordinate.

TODO: write tests
*/

use crate::common::{Coordinate, Count, PointCount};
use crate::line::LineIterator;

use std::fmt;

pub struct Grid {
    cells: Vec<Vec<usize>>,
    max_row: usize,
    max_col: usize,
}

impl Grid {
    pub fn new(lines: Vec<(Coordinate, Coordinate)>) -> Self {
        let mut grid = Self {
            cells: vec![vec![0usize; 2]; 2],
            max_row: 1,
            max_col: 1,
        };
        for line in lines {
            grid.add_line(line.0, line.1);
        }
        grid
    }

    fn add_line(&mut self, start: Coordinate, end: Coordinate) {
        let line_coordinates: Vec<Coordinate> = LineIterator::new(start, end).collect();
        for point in line_coordinates {
            self.add_point(point);
        }
    }

    fn add_point(&mut self, point: Coordinate) {
        self.max_row = self.max_row.max(point.row);
        self.max_col = self.max_col.max(point.col);

        if self.cells.len() <= self.max_row {
            self.cells
                .resize_with(self.max_row + 1, || vec![0; self.max_col + 1]);
        }
        if self.cells[self.max_row].len() <= self.max_col {
            for row in &mut self.cells {
                row.resize(self.max_col + 1, 0);
            }
        }

        self.cells[point.row][point.col] += 1;
    }

    pub fn get_count(&self, point: &Coordinate) -> Option<PointCount> {
        if point.row <= self.max_row && point.col <= self.max_col {
            Some(PointCount {
                point: *point,
                count: Count(self.cells[point.row][point.col]),
            })
        } else {
            None
        }
    }

}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for &cell in row {
                if cell == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", cell)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_point_1_1() {
        let grid = Grid::new(vec![(Coordinate { row: 1, col: 1 }, Coordinate { row: 1, col: 1 })]);
        let point = Coordinate { row: 1, col: 1 };
        assert_eq!(grid.get_count(&point).unwrap().count.0, 1);
    }

    #[test]
    fn test_get_count_1_1() {
        let grid = Grid::new(vec![(Coordinate { row: 1, col: 1 }, Coordinate { row: 1, col: 1 })]);
        let point = Coordinate { row: 1, col: 1 };
        assert_eq!(grid.get_count(&point).unwrap().count.0, 1);
    }

    #[test]
    fn test_add_multiple_points_same_coordinates() {
        let grid = Grid::new(vec![
            (Coordinate { row: 1, col: 1 }, Coordinate { row: 1, col: 1 }),
            (Coordinate { row: 1, col: 1 }, Coordinate { row: 1, col: 1 }),
        ]);
        let point = Coordinate { row: 1, col: 1 };
        assert_eq!(grid.get_count(&point).unwrap().count.0, 2);
    }

    #[test]
    fn test_add_points_different_coordinates() {
        let grid = Grid::new(vec![
            (Coordinate { row: 1, col: 1 }, Coordinate { row: 1, col: 1 }),
            (Coordinate { row: 2, col: 2 }, Coordinate { row: 2, col: 2 }),
        ]);
        let point1 = Coordinate { row: 1, col: 1 };
        let point2 = Coordinate { row: 2, col: 2 };
        assert_eq!(grid.get_count(&point1).unwrap().count.0, 1);
        assert_eq!(grid.get_count(&point2).unwrap().count.0, 1);
    }

    #[test]
    fn test_get_count_point_not_added() {
        let grid = Grid::new(vec![]);
        assert_eq!(
            grid.get_count(&Coordinate { row: 1, col: 1 })
                .unwrap()
                .count
                .0,
            0
        );
    }
}