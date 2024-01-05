/// A Grid struct represents a grid/matrix.
/// It provides methods to add lines and points to the grid,
/// resize the grid, and keep count the number of occurances
/// at any point in the grid.
///
/// The print output from the Display trait for Grid struct is
/// the visual Solution to part 1.
use std::fmt;

use crate::common::{Coordinate, Count, PointCount};
use crate::file::PointsData;
use crate::line::LineIterator;

pub struct Grid {
    cells: Vec<Vec<usize>>,
}

impl Grid {
    pub fn new(points: PointsData) -> Self {
        let mut grid = Self {
            cells: vec![vec![0usize; points.max_x + 1]; points.max_y + 1],
        };
        points.iter().for_each(|coordinate| {
            grid.add_line(coordinate.0, coordinate.1);
        });
        grid
    }

    fn add_point(&mut self, point: Coordinate) {
        self.cells[point.y][point.x] += 1;
    }

    fn add_line(&mut self, start: Coordinate, end: Coordinate) {
        LineIterator::new(start, end)
            .collect::<Vec<Coordinate>>()
            .iter()
            .for_each(|point| {
                self.add_point(*point);
            });
    }

    pub fn get_count(&self, point: &Coordinate) -> Option<PointCount> {
        if point.y < self.cells.len() && point.x < self.cells[point.y].len() {
            Some(PointCount {
                point: *point,
                count: Count(self.cells[point.y][point.x]),
            })
        } else {
            None
        }
    }

    pub fn sum_double_counts(&self) -> i32 {
        self.cells
            .iter()
            .flatten()
            .filter(|&&count| count >= 2)
            .count() as i32
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // writeln!(f, "Grid:")?;
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
    use crate::file::PointsData;

    #[test]
    fn test_grid_new() {
        let points_data = PointsData {
            point_pairs: vec![
                (Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 1 }),
                (Coordinate { x: 0, y: 1 }, Coordinate { x: 1, y: 1 }),
            ],
            max_x: 1,
            max_y: 1,
        };
        let grid = Grid::new(points_data);
        assert_eq!(grid.cells.len(), 2);
        assert_eq!(grid.cells[0].len(), 2);
    }

    #[test]
    fn test_grid_get_count() {
        let points_data = PointsData {
            point_pairs: vec![
                (Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 1 }),
                (Coordinate { x: 2, y: 2 }, Coordinate { x: 3, y: 3 }),
            ],
            max_x: 1,
            max_y: 1,
        };
        let grid = Grid::new(points_data);
        let count_origin = grid.get_count(&Coordinate { x: 0, y: 0 });
        assert_eq!(
            count_origin,
            Some(PointCount {
                point: Coordinate { x: 0, y: 0 },
                count: Count(1)
            })
        );
    }
}
