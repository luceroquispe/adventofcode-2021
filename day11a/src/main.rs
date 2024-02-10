use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct DumboOctopus {
    grid: Vec<Vec<u8>>,
    flash_count: u64,
    step_count: u64,
}

impl DumboOctopus {
    fn new(grid: &[Vec<u8>]) -> Self {
        DumboOctopus {
            grid: grid.to_vec(),
            flash_count: 0,
            step_count: 0,
        }
    }

    fn find_adjacent_neighbours(&mut self) {
        unimplemented!()
    }
}


// useful in function signatures where you want to accept different
// types of arguments as long as they can be referenced as a certain type.
// In the context of file paths, AsRef<Path> allows the function to accept
// any type that can be converted into a Path reference.
fn read_to_2d_array<P: AsRef<Path>>(path: P) -> io::Result<Vec<Vec<u8>>> {
    let file = File::open(path)?;
    let buf_reader = io::BufReader::new(file);
    let array_2d = buf_reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    Ok(array_2d)
}

fn step_increase_energy_levels(arr: &mut [Vec<u8>]) {
    arr.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|col| {
            *col += 1;
        });
    });
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
        let mut result = read_to_2d_array(path).expect("Failed to read 2D array from file");
        let _ = step_increase_energy_levels(&mut result);
        assert_eq!(result, expected_array_2d);
    }
}
