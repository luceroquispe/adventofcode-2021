/// This file defines common types used across the project.
/// It defines a Coordinate struct that represents a point in a 2D space,
/// a Count struct that represents a count of a point,
/// and a PointCount struct that represents a point and its count.
///
/// It also implements the FromStr trait for the Coordinate struct to
/// parse a Coordinate from a string, and the Display trait for the
/// Count and PointCount structs to provide a custom format for printing
/// these types and checking.
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::common::Coordinate;

pub struct PointsData {
    pub point_pairs: Vec<(Coordinate, Coordinate)>,
    pub max_x: usize,
    pub max_y: usize,
}

pub struct PointsDataIter<'a> {
    points_data: &'a PointsData,
    index: usize,
}

impl PointsData {
    pub fn iter(&self) -> PointsDataIter {
        PointsDataIter {
            points_data: self,
            index: 0,
        }
    }
}

impl<'a> Iterator for PointsDataIter<'a> {
    type Item = &'a (Coordinate, Coordinate);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.points_data.point_pairs.len() {
            let result = &self.points_data.point_pairs[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

fn parse_line(input: &str) -> Option<(Coordinate, Coordinate)> {
    let coords: Vec<&str> = input.trim().split("->").collect();
    if coords.len() != 2 {
        return None;
    }
    let start: Coordinate = coords[0].parse().ok()?;
    let end: Coordinate = coords[1].parse().ok()?;

    if (start.x != end.x) && (start.y != end.y) {
        return None;
    }

    Some((start, end))
}

pub fn read_file_to_points(path: &str) -> io::Result<PointsData> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let point_pairs: Vec<_> = reader
        .lines()
        .filter_map(|line| line.ok().as_deref().and_then(parse_line))
        .collect();

    let (max_x, max_y) = point_pairs
        .iter()
        .fold((0, 0), |(max_x, max_y), (start, end)| {
            (max_x.max(start.x).max(end.x), max_y.max(start.y).max(end.y))
        });

    Ok(PointsData {
        point_pairs,
        max_x,
        max_y,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_coordinate_from_str_ok() {
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
    fn test_parse_line_ok() {
        let coord_str = String::from("0,9 -> 2,9");
        let coords = parse_line(&coord_str);
        assert_eq!(
            coords.unwrap(),
            (Coordinate { x: 0, y: 9 }, Coordinate { x: 2, y: 9 })
        );
    }

    #[test]
    fn test_parse_line_return_none() {
        let coord_str = String::from("0,9 -> 2,8"); // not horizontal or vertical
        let coords = parse_line(&coord_str);
        assert!(coords.is_none());
    }

    #[test]
    fn test_read_file_to_points_ok() {
        let path = "/tmp/test_read_file_to_points_ok.txt";
        let mut file = File::create(path).unwrap();
        writeln!(file, "0,0 -> 0,1").unwrap();
        writeln!(file, "0,1 -> 1,1").unwrap();

        let points_data = read_file_to_points(path).unwrap();
        assert_eq!(points_data.point_pairs.len(), 2);
        assert_eq!(points_data.max_x, 1);
        assert_eq!(points_data.max_y, 1);

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_read_file_to_points_empty_file() {
        let path = "/tmp/test_read_file_to_points_empty_file.txt";
        let _ = File::create(path).unwrap();

        let points_data = read_file_to_points(path).unwrap();
        assert_eq!(points_data.point_pairs.len(), 0);
        assert_eq!(points_data.max_x, 0);
        assert_eq!(points_data.max_y, 0);

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_read_file_to_points_nonexistent_file() {
        let path = "/tmp/nonexistent_file.txt";
        assert!(Path::new(path).exists() == false);

        let result = read_file_to_points(path);
        assert!(result.is_err());

        let _ = std::fs::remove_file(path);
    }
}
