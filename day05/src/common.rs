use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
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

pub struct Count(pub usize);

impl fmt::Display for Count {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Count: {}", self.0)
    }
}

pub struct PointCount {
    pub point: Coordinate,
    pub count: Count,
}

impl fmt::Display for PointCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Point ({},{}): Count={}",
            self.point.x, self.point.y, self.count.0
        )
    }
}

pub struct Line {
    pub start: Coordinate,
    pub end: Coordinate,
}

impl Line {
    pub fn new(start: Coordinate, end: Coordinate) -> Self {
        Self { start, end }
    }
}


