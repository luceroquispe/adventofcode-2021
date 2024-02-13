use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]
struct Coordinate {
    row: u8,
    col: u8,
}

#[derive(PartialEq, Debug)]
struct DumboOctopus {
    grid: Vec<Vec<i8>>,
    flash_count: i64,
    step_count: i64,
    flash_stack: Vec<Coordinate>,
}

impl DumboOctopus {
    fn new(grid: &[Vec<i8>]) -> Self {
        DumboOctopus {
            grid: grid.to_vec(),
            flash_count: 0,
            step_count: 0,
            flash_stack: Vec::new(),
        }
    }

    fn increment_step_and_neighbouring_flash(&mut self, coord: Coordinate) {
        // Define the directions for adjacent neighbors
        let directions: Vec<(isize, isize)> = vec![
            (-1, -1), // BottomLeft
            (-1, 0),  // Bottom
            (-1, 1),  // BottomRight
            (0, 1),   // Right
            (1, 1),   // TopRight
            (1, 0),   // Top
            (1, -1),  // TopLeft
            (0, -1),  // Left
        ];

        for (row, col) in directions {
            let current_row = coord.row;
            let current_col = coord.col;
            // Convert coord to isize to prevent underflow when adding negative numbers
            let new_row = coord.row as isize + row;
            let new_col = coord.col as isize + col;

            // Check if the new coordinates are within the grid boundaries
            if new_row >= 0
                && new_col >= 0
                && (new_row as usize) < self.grid.len()
                && (new_col as usize) < self.grid[0].len()
            {
                // The almost flashed neighbour will increment current
                if self.grid[new_row as usize][new_col as usize] == 9 {
                    // step
                    self.grid[current_row as usize][current_col as usize] += 1;
                    // These need to be reset to 0 later
                    self.flash_stack.push(Coordinate {
                        row: new_row as u8,
                        col: new_col as u8,
                    })
                }
            }
        }
    }

    fn step_increase_energy_levels(&mut self) {
        let rows = self.grid.len();
        let cols = if rows > 0 { self.grid[0].len() } else { 0 };

        for r_idx in 0..rows {
            for c_idx in 0..cols {
                self.increment_step_and_neighbouring_flash(Coordinate {
                    row: r_idx as u8,
                    col: c_idx as u8,
                });
            }
        }
    }

    fn reset_to_zero_after_flash(&mut self) {
        while let Some(coord) = self.flash_stack.pop() {
            self.grid[coord.row as usize][coord.col as usize] = 0;
        }
    }
}
// AsRef are useful in function signatures where you want to accept different
// types of arguments as long as they can be referenced as a certain type.
// In the context of file paths, AsRef<Path> allows the function to accept
// any type that can be converted into a Path reference.
fn read_to_2d_array<P: AsRef<Path>>(path: P) -> io::Result<Vec<Vec<i8>>> {
    let file = File::open(path)?;
    let buf_reader = io::BufReader::new(file);
    let array_2d = buf_reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>();
    Ok(array_2d)
}

fn main() -> io::Result<()> {
    let path = "data/step_0.txt";
    let array_2d = read_to_2d_array(path)?;
    // Just for demonstration, print the 2D array
    for row in array_2d {
        println!("{:?}", row);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_increase_energy_adds_one_ok() {
        let expected_path = "data/step_1.txt";
        let expected_array_2d =
            read_to_2d_array(expected_path).expect("Failed to read expected 2D array from file");

        let path = "data/step_0.txt";
        let grid = read_to_2d_array(path).expect("Failed to read 2D array from file");
        let mut result = DumboOctopus::new(&grid);
        result.step_increase_energy_levels();
        assert_eq!(result.grid, expected_array_2d);
    }
}
