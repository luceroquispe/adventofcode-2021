/// This module provides functionality to parse and handle Coordinates from a file.
///
/// It defines a `Coordinate` struct and implements the `FromStr` trait for it,
/// allowing coordinates to be parsed from strings.
/// It also provides a `parse_line` function for parsing a line of input into
/// a vector of `Coordinate` instances, and a `read_file_to_points` function
/// for reading a file of lines into a vector of `Coordinate` instances.
///
/// The module also includes tests for:
///     * `Coordinate` struct
///     * `parse_line` function
///     * `parse_file_to_points_pairs`
///     * `read_file_to_points` function
///
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::common::Coordinate;

fn parse_line(input: &str) -> Option<(Coordinate, Coordinate)> {
    let coords: Vec<&str> = input.split("->").collect();
    if coords.len() != 2 {
        return None;
    }

    let start: Coordinate = coords[0].trim().parse().ok()?;
    let end: Coordinate = coords[1].trim().parse().ok()?;

    if start.row == end.row || start.col == end.col {
        Some((start, end))
    } else {
        None
    }
}

pub fn read_file_to_points(path: &str) -> io::Result<Vec<(Coordinate, Coordinate)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let point_pairs: Vec<_> = reader
        .lines()
        .filter_map(|line| line.ok().and_then(|l| parse_line(&l)))
        .collect();
    Ok(point_pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_from_str() {
        // Test case 1
        let coord_str = "0,9";
        let coord = coord_str.parse::<Coordinate>();
        assert!(coord.is_ok());
        assert_eq!(coord.unwrap(), Coordinate { row: 0, col: 9 });

        // Test case 2
        let coord_str = "5,9";
        let coord = coord_str.parse::<Coordinate>();
        assert!(coord.is_ok(), "Failed to parse coordinate: {}", coord_str);
        assert_eq!(
            coord.unwrap(),
            Coordinate { row: 5, col: 9 },
            "Coordinate parsed but not equal to expected value"
        );
    }

    #[test]
    fn test_parse_line_ok() {
        // Test case 1
        let line = "0,9 -> 5,9";
        let result = parse_line(line);
        assert_eq!(
            result.unwrap(),
            (Coordinate { row: 0, col: 9 }, Coordinate { row: 5, col: 9 }),
            "Failed to parse line: {}",
            line
        )
    }

    #[test]
    fn test_parse_line_return_none() {
        // Test case 2
        let line = "8,0 -> 0,8";
        let result = parse_line(line);
        assert!(result.is_none(), "Expected None, but got Some");
    }

    #[test]
    fn test_read_file_to_points_dropping_diagonals() {
        // missing a few records since they aren't straight lines
        let path = "data/sample1.txt";
        let result = read_file_to_points(&path.to_string());
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                (Coordinate { row: 0, col: 9 }, Coordinate { row: 5, col: 9 }),
                (Coordinate { row: 9, col: 4 }, Coordinate { row: 3, col: 4 }),
                (Coordinate { row: 2, col: 2 }, Coordinate { row: 2, col: 1 }),
                (Coordinate { row: 7, col: 0 }, Coordinate { row: 7, col: 4 }),
                (Coordinate { row: 0, col: 9 }, Coordinate { row: 2, col: 9 }),
                (Coordinate { row: 3, col: 4 }, Coordinate { row: 1, col: 4 })]
        )
    }
}
