#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offsets(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

struct HeightMap {
    map: Vec<Vec<u32>>,
}

impl HeightMap {
    fn new(map: Vec<Vec<u32>>) -> Self {
        HeightMap { map }
    }

    fn find_low_points(&self) -> Vec<u32> {
        let mut low_points = Vec::new();
        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

        for (i, row) in self.map.iter().enumerate() {
            println!("{:?}", row);
            for (j, &height) in row.iter().enumerate() {
                let is_low_point = directions.iter().all(|&direction| {
                    let (di, dj) = direction.offsets();
                    let new_i = i as i32 + di;
                    let new_j = j as i32 + dj;
                
                    if new_i >= 0 && new_i < self.map.len() as i32 && new_j >= 0 && new_j < self.map[new_i as usize].len() as i32 {
                        self.map[new_i as usize][new_j as usize] > height
                    } else {
                        true
                    }
                });

                if is_low_point {
                    low_points.push(height);
                }
            }
        }
        low_points
    }

    fn calculate_risk_level_sum(&self) -> u32 {
        self.find_low_points().iter().map(|&height| height + 1).sum()
    }
}

fn main() {
    let heightmap = HeightMap::new(vec![
        vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
        vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
        vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
        vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
        vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
    ]);

    let risk_level_sum = heightmap.calculate_risk_level_sum();
    println!("The sum of the risk levels of all low points is: {}", risk_level_sum);
}

#[cfg(test)]
mod tests {
    use super::*; // Import the necessary structs and enums from the outer module

    #[test]
    fn test_heightmap_initialization() {
        let heightmap = HeightMap::new(vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]);

        assert_eq!(heightmap.map[0][1], 1, "HeightMap initialization failed at [0][1]");
        // Add more assertions as needed to test various points...
    }

    #[test]
    fn test_direction_offsets() {
        assert_eq!(Direction::Up.offsets(), (-1, 0), "Up direction offsets are incorrect");
        assert_eq!(Direction::Down.offsets(), (1, 0), "Down direction offsets are incorrect");
        // Test the rest of the directions...
    }

    #[test]
    fn test_adjacent_location_checks_on_corners() {
        let heightmap = HeightMap::new(vec![
            vec![1, 2, 1],
            vec![2, 3, 2],
            vec![1, 2, 1],
        ]);

        let low_points = heightmap.find_low_points();
        assert!(low_points.contains(&1), "Failed to identify low points correctly");
        assert_eq!(low_points.len(), 4, "Incorrect number of low points identified");
    }

    #[test]
    fn test_risk_level_calculation() {
        let heightmap = HeightMap::new(vec![
            vec![1, 2, 1],
            vec![2, 3, 2],
            vec![1, 2, 1],
        ]);

        let risk_level_sum = heightmap.calculate_risk_level_sum();
        assert_eq!(risk_level_sum, 8, "Risk level sum calculation is incorrect");
    }
}
