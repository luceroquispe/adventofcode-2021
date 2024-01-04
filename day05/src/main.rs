/*
TODO: Read file into points
*/
use day05::file::read_file_to_points;
use day05::grid::Grid;

fn main() {
    let path = String::from("data/sample1.txt");
    let points = read_file_to_points(&path).unwrap();
    print!("{:?}", points);
    let grid = Grid::new(points);
    println!("{}", grid)
}
