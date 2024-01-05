use day05::file::read_file_to_points;
use day05::grid::Grid;

fn main() {
    let path = String::from("data/data1.txt");
    let points = read_file_to_points(&path).unwrap();
    let grid = Grid::new(points);
    // println!("{}", grid)
    print!("{}", grid.sum_double_counts())
}
