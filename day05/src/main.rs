use std::fmt;

#[derive(Clone, Copy)]
pub struct Point {
    row: usize,
    col: usize,
}

pub struct Count(pub usize);

impl fmt::Display for Count {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Count: {}", self.0)
    }
}

pub struct PointCount {
    point: Point,
    count: Count,
}

impl fmt::Display for PointCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Point ({},{}): Count={}",
            self.point.row, self.point.col, self.count.0
        )
    }
}

pub struct Grid {
    cells: Vec<Vec<usize>>,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            cells: vec![vec![0; 9]; 9],
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Grid:")?;
        for row in &self.cells {
            write!(f, "[")?;
            for (j, cell) in row.iter().enumerate() {
                write!(f, "{}", cell)?;
                if j != row.len() - 1 {
                    write!(f, ", ")?;
                }
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

pub trait GridOperations {
    fn add_point(&mut self, point: Point);
    fn get_count(&self, point: &Point) -> Option<PointCount>;
}

impl GridOperations for Grid {
    fn add_point(&mut self, point: Point) {
        self.cells[point.row][point.col] += 1;
    }

    fn get_count(&self, point: &Point) -> Option<PointCount> {
        Some(PointCount {
            point: *point,
            count: Count(self.cells[point.row][point.col]),
        })
    }
}

fn main() {
    let mut grid = Grid::default();
    let point = Point { row: 1, col: 1 };
    grid.add_point(point);
    println!("{}", grid);
    let count = grid.get_count(&point).unwrap();
    println!("{}", count);
}
