/* 
Read file of lines into vector.

For example:

    0,9 -> 5,9
    8,0 -> 0,8

goes to:

    vec![(0,9), (5,9), (8,0), (0,8)]

TODO: write tests
*/
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;


pub struct Coordinate {
    x: i32,
    y: i32,
}

impl FromStr for Coordinate {
    type Err = std::num::ParseIntError;
 
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        let x = parts[0].parse()?;
        let y = parts[1].parse()?;
        Ok(Coordinate { x, y })
    }
 }

 fn parse_line(input: String) -> Result<Vec<Coordinate>, std::num::ParseIntError> {
    input
        .split("->")
        .map(|coords| coords.trim())
        .filter(|coords| !coords.is_empty())
        .map(|coords| coords.parse())
        .collect()
 }
 
 fn parse_file_to_points_pairs(reader: BufReader<File>) -> Result<Vec<Vec<Coordinate>>, Box<dyn std::error::Error>> {
    let mut point_pairs = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let points = parse_line(line)?;
        point_pairs.push(points);
    }
    Ok(point_pairs)
 }
 
 pub fn read_file_to_points(path: &String) -> Result<Vec<Coordinate>, Box<dyn std::error::Error>> {
    let mut coordinates = Vec::<Coordinate>::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);
 
    for line_result in reader.lines() {
        let line = line_result?;
        let coordinate: Coordinate = line.trim().parse()?;
        coordinates.push(coordinate);
    }
 
    Ok(coordinates)
 }
 