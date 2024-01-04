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
            x: if start.col <= end.col { 1 } else { -1 },
            y: if start.row <= end.row { 1 } else { -1 },
        };
        Self {
            current: start,
            end,
            step,
            done: false,
        }
    }

    fn update_current(&mut self) {
        if self.current.col != self.end.col {
            self.current.col = (self.current.col as i32 + self.step.x) as usize;
        }
        if self.current.row != self.end.row {
            self.current.row = (self.current.row as i32 + self.step.y) as usize;
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
        let line = Line::new(Coordinate { col: 0, row: 0 }, Coordinate { col: 5, row: 0 });
        let expected_coordinates = vec![
            Coordinate { col: 0, row: 0 },
            Coordinate { col: 1, row: 0 },
            Coordinate { col: 2, row: 0 },
            Coordinate { col: 3, row: 0 },
            Coordinate { col: 4, row: 0 },
            Coordinate { col: 5, row: 0 },
        ];
        assert_eq!(line.line_coordinates, expected_coordinates);
    }

    #[test]
    fn test_to_line_coords_unordered() {
        let line = Line::new(Coordinate { col: 5, row: 1 }, Coordinate { col: 1, row: 1 });
        let expected_coordinates = vec![
            Coordinate { col: 5, row: 1 },
            Coordinate { col: 4, row: 1 },
            Coordinate { col: 3, row: 1 },
            Coordinate { col: 2, row: 1 },
            Coordinate { col: 1, row: 1 },
        ];
        assert_eq!(line.line_coordinates, expected_coordinates);
    }
}
