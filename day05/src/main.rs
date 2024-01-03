/*
TODO: Read file into points
*/

use day05::grid::{Grid, GridOperations, Point};

fn main() {
    let mut grid = Grid::default();
    let point = Point { row: 1, col: 1 };
    grid.add_point(point);
    println!("{}", grid);
    let count = grid.get_count(&point).unwrap();
    println!("{}", count);
}
