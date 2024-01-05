///
/// This file defines a Line struct and a LineIterator struct.
/// The Line struct represents a line with a start and end coordinate,
/// and a list of all coordinates that the line passes through.
///
/// The LineIterator struct is an iterator that generates all coordinates
/// from the start to the end of a line. Order doesn't matter.
use crate::common::Coordinate;

pub struct Step {
    x: i32,
    y: i32,
}

pub struct LineIterator {
    current: Coordinate,
    end: Coordinate,
    step: Step,
    done: bool,
}

impl LineIterator {
    pub fn new(start: Coordinate, end: Coordinate) -> Self {
        let step = Step {
            x: if start.x <= end.x { 1 } else { -1 },
            y: if start.y <= end.y { 1 } else { -1 },
        };
        Self {
            current: start,
            end,
            step,
            done: false,
        }
    }

    fn update_current(&mut self) {
        if self.current.x != self.end.x {
            self.current.x = (self.current.x as i32 + self.step.x) as usize;
        }
        if self.current.y != self.end.y {
            self.current.y = (self.current.y as i32 + self.step.y) as usize;
        }
    }
}

impl Iterator for LineIterator {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else if self.current != self.end {
            let result = self.current;
            self.update_current();
            Some(result)
        } else {
            self.done = true;
            Some(self.current)
        }
    }
}
pub struct Line {
    pub start: Coordinate,
    pub end: Coordinate,
    pub line_coordinates: Vec<Coordinate>,
}

impl Line {
    pub fn new(start: Coordinate, end: Coordinate) -> Self {
        let line_coordinates: Vec<Coordinate> = LineIterator::new(start, end).collect();
        Self {
            start,
            end,
            line_coordinates,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_line_coords_starting_at_zero() {
        let line = Line::new(Coordinate { x: 0, y: 0 }, Coordinate { x: 5, y: 0 });
        let expected_coordinates = vec![
            Coordinate { x: 0, y: 0 },
            Coordinate { x: 1, y: 0 },
            Coordinate { x: 2, y: 0 },
            Coordinate { x: 3, y: 0 },
            Coordinate { x: 4, y: 0 },
            Coordinate { x: 5, y: 0 },
        ];
        assert_eq!(line.line_coordinates, expected_coordinates);
    }

    #[test]
    fn test_to_line_coords_unordered() {
        let line = Line::new(Coordinate { x: 5, y: 1 }, Coordinate { x: 1, y: 1 });
        let expected_coordinates = vec![
            Coordinate { x: 5, y: 1 },
            Coordinate { x: 4, y: 1 },
            Coordinate { x: 3, y: 1 },
            Coordinate { x: 2, y: 1 },
            Coordinate { x: 1, y: 1 },
        ];
        assert_eq!(line.line_coordinates, expected_coordinates);
    }
}
