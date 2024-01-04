/*
TODO: Read file into points
*/
use day05::common::Coordinate;
use day05::grid::{Grid, GridOperations};
use day05::file::read_file_to_points;

fn main() {
    let mut grid = Grid::default();
    let point = Coordinate { x: 1, y: 1 };
    grid.add_point(point);
    println!("{}", grid);
    let count = grid.get_count(&point).unwrap();
    println!("{}", count);
    let path = String::from("data/sample1.txt");
    let points = read_file_to_points(&path).unwrap();
    print!("{:?}", points);

}
