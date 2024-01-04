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
///     * `read_file_to_points` function
///
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::common::Coordinate;

fn parse_line(input: String) -> Result<Vec<Coordinate>, std::num::ParseIntError> {
    input
        .split("->")
        .map(|coords| coords.trim())
        .filter(|coords| !coords.is_empty())
        .map(|coords| coords.parse())
        .collect()
}

fn parse_file_to_points_pairs(
    reader: BufReader<File>,
) -> Result<Vec<Vec<Coordinate>>, Box<dyn std::error::Error>> {
    let mut point_pairs = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let points = parse_line(line)?;
        point_pairs.push(points);
    }
    Ok(point_pairs)
}

pub fn read_file_to_points(
    path: &String,
) -> Result<Vec<Vec<Coordinate>>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    parse_file_to_points_pairs(reader)
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
        assert_eq!(coord.unwrap(), Coordinate { x: 0, y: 9 });

        // Test case 2
        let coord_str = "5,9";
        let coord = coord_str.parse::<Coordinate>();
        assert!(coord.is_ok(), "Failed to parse coordinate: {}", coord_str);
        assert_eq!(
            coord.unwrap(),
            Coordinate { x: 5, y: 9 },
            "Coordinate parsed but not equal to expected value"
        );
    }

    #[test]
    fn test_parse_line() {
        // Test case 1
        let line = "0,9 -> 5,9";
        let result = parse_line(line.to_string());
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![Coordinate { x: 0, y: 9 }, Coordinate { x: 5, y: 9 }],
            "Failed to parse line: {}",
            line
        );

        // Test case 2
        let line = "8,0 -> 0,8";
        let result = parse_line(line.to_string());
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![Coordinate { x: 8, y: 0 }, Coordinate { x: 0, y: 8 }],
            "Failed to parse line: {}",
            line
        );
    }

    #[test]
    fn test_read_file_to_points() {
        // Test case 1
        let path = "data/sample1.txt";
        let result = read_file_to_points(&path.to_string());
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                [Coordinate { x: 0, y: 9 }, Coordinate { x: 5, y: 9 }],
                [Coordinate { x: 8, y: 0 }, Coordinate { x: 0, y: 8 }],
                [Coordinate { x: 9, y: 4 }, Coordinate { x: 3, y: 4 }],
                [Coordinate { x: 2, y: 2 }, Coordinate { x: 2, y: 1 }],
                [Coordinate { x: 7, y: 0 }, Coordinate { x: 7, y: 4 }],
                [Coordinate { x: 6, y: 4 }, Coordinate { x: 2, y: 0 }],
                [Coordinate { x: 0, y: 9 }, Coordinate { x: 2, y: 9 }],
                [Coordinate { x: 3, y: 4 }, Coordinate { x: 1, y: 4 }],
                [Coordinate { x: 0, y: 0 }, Coordinate { x: 8, y: 8 }],
                [Coordinate { x: 5, y: 5 }, Coordinate { x: 8, y: 2 }]
            ]
        )
    }
}
