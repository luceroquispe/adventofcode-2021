use std::fmt;

#[derive(Debug)]
struct Fish {
    days_until_spawn: i32,
}

impl Fish {
    fn is_spawning(&self) -> bool {
        self.days_until_spawn == 0
    }
}

#[derive(Debug)]
struct FishSchool {
    fish: Vec<Fish>,
}
impl FishSchool {
    fn new(fish_vec: Vec<i32>) -> Self {
        let fish: Vec<Fish> = fish_vec
            .iter()
            .map(|d| Fish {
                days_until_spawn: *d,
            })
            .collect();
        FishSchool { fish }
    }

    fn pass_day(&mut self) {
        let mut new_fish: Vec<Fish> = Vec::new();
        for fish in &mut self.fish {
            if !fish.is_spawning() {
                fish.days_until_spawn -= 1
            } else {
                new_fish.push(Fish {
                    days_until_spawn: 8,
                });
                fish.days_until_spawn = 6;
            }
        }
        self.fish.extend(new_fish);
    }

    fn pass_days(&mut self, days: i32) {
        for _ in 1..=days {
            self.pass_day()
        }
    }
    fn count(&self) -> usize {
        self.fish.len()
    }
}

impl fmt::Display for FishSchool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FishSchool [")?;
        for (i, fish) in self.fish.iter().enumerate() {
            write!(f, "{}", fish.days_until_spawn)?;
            if i != self.fish.len() - 1 {
                write!(f, ", ")?
            }
        }
        write!(f, "]")
    }
}

fn main() {
    // part 1
    let mut fish_school = FishSchool::new(vec![
        1, 1, 1, 1, 3, 1, 4, 1, 4, 1, 1, 2, 5, 2, 5, 1, 1, 1, 4, 3, 1, 4, 1, 1, 1, 1, 1, 1, 1, 2,
        1, 2, 4, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 5, 1, 1, 2, 1, 5, 1, 1, 1, 1, 1, 1, 1, 1, 4, 3, 1,
        1, 1, 2, 1, 1, 5, 2, 1, 1, 1, 1, 4, 5, 1, 1, 2, 4, 1, 1, 1, 5, 1, 1, 1, 1, 5, 1, 3, 1, 1,
        4, 2, 1, 5, 1, 2, 1, 1, 1, 1, 1, 3, 3, 1, 5, 1, 1, 1, 1, 3, 1, 1, 1, 4, 1, 1, 1, 4, 1, 4,
        3, 1, 1, 1, 4, 1, 2, 1, 1, 1, 2, 1, 1, 1, 1, 5, 1, 1, 3, 5, 1, 1, 5, 2, 1, 1, 1, 1, 1, 4,
        4, 1, 1, 2, 1, 1, 1, 4, 1, 1, 1, 1, 5, 3, 1, 1, 1, 5, 1, 1, 1, 4, 1, 4, 1, 1, 1, 5, 1, 1,
        3, 2, 2, 1, 1, 1, 4, 1, 3, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 4, 1, 1, 1, 1, 2, 1, 4, 1, 1, 1,
        1, 1, 4, 1, 1, 2, 4, 2, 1, 2, 3, 1, 3, 1, 1, 2, 1, 1, 1, 3, 1, 1, 3, 1, 1, 4, 1, 3, 1, 1,
        2, 1, 1, 1, 4, 1, 1, 3, 1, 1, 5, 1, 1, 3, 1, 1, 1, 1, 5, 1, 1, 1, 1, 1, 2, 3, 4, 1, 1, 1,
        1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 3, 2, 2, 1, 3, 5, 1, 1, 4, 4, 1, 3, 4, 1, 2, 4, 1, 1, 3, 1,
    ]);
    fish_school.pass_days(80);
    println!("Part One solution: {}", fish_school.count());

    // part 2 using same instance
    fish_school.pass_days(256-80);
    println!("Part Two solution: {}", fish_school.count())
}
