use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coordinate {
    pub y: usize,
    pub x: usize,
}

impl FromStr for Coordinate {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split(',').collect();
        let col = parts[0].parse()?;
        let row = parts[1].parse()?;
        Ok(Coordinate { x: col, y: row })
    }
}

#[derive(PartialEq, Debug)]
pub struct Count(pub usize);

impl fmt::Display for Count {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Count: {}", self.0)
    }
}
#[derive(PartialEq, Debug)]
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
