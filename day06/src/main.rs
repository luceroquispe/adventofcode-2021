/// Represents a school of fish.
/// Each fish has an internal timer which determines its state.
/// When a fish's timer reaches 0, it creates a new fish with an internal timer of 8.
/// All fish are contained in a vector.
struct FishSchool {
    fish: Vec<usize>,
 }
 
 impl FishSchool {
    /// Creates a new FishSchool with the given initial fish.
    fn new(initial_fish: Vec<usize>) -> Self {
        FishSchool { fish: initial_fish }
    }
 
    /// Simulates the growth of the fish school over a certain number of days.
    /// Returns the total number of fish after the simulation.
    fn simulate_fishes(&mut self, days: usize) -> usize {
        // Create a map to count the number of fish at each stage of their lifecycle.
        // Index 0 represents fish with a timer of 0, index 1 represents fish with a timer of 1, etc.
        let mut map = [0; 9];
        // Count the number of fish and their life cycle at the start.
        // It does this by incrementing the corresponding index in the map array for each fish. 
        // For instance, if the fish vector is:
        //      [3, 4, 3, 1, 2], 
        // after this loop, the map array would look like this: 
        //      [0, 0, 0, 2, 1, 1, 0, 0, 0]. 
        // This means there are 2 fish with a timer of 3, 1 fish with a timer of 4, and 1 fish with a timer of 1.
        for &fish in &self.fish {
            map[fish] += 1;
        }
 
        // Simulate each day.
        for _ in 1..days {
            // Each day, all fish with a timer of 0 create a new fish with a timer of 8.
            // These new fish are added to the map at index 8.
            // say after some time, we have a usize array [1,0,0,0,0,0,0,0,0]. 
            // Here, there is 1 fish with a timer of 0. So, map[0] would be 1. 
            // The line map[8] += map[0]; now adds the value of map[0] (which is 1) 
            // to map[8]. So, the array becomes [1,0,0,0,0,0,0,0,1]. 
            // This means that 1 new fish with a timer of 8 was created.
            map[8] += map[0];
            // Then, all fish move forward one stage in their lifecycle.
            // This is done by shifting all elements in the map one position to the left,
            // wrapping around from index 0 to index 8.
            map.rotate_left(1);
        }
 
        // Sum up the counts in the map to get the total number of fish.
        map.iter().sum()
    }
 }
 
 fn main() {
    let mut fish = FishSchool::new(vec![
        // Initial fish population...
    ]);
    // part 1
    println!("Part 1: {:?}", fish.simulate_fishes(80));
    // part 2
    println!("Part 2: {:?}", fish.simulate_fishes(256))
 }
