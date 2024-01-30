fn main() {
    let input = include_str!("../data/data1.txt");
    let sum: u32 = input.lines().map(|line| decode_line(line)).sum();
    println!("Sum of all output values: {}", sum);
}

fn decode_line(line: &str) -> u32 {
    let parts: Vec<&str> = line.split(" | ").collect();
    let patterns: Vec<&str> = parts[0].split_whitespace().collect();
    let outputs: Vec<&str> = parts[1].split_whitespace().collect();

    // Map of digit to its signal pattern (sorted for easy comparison)
    let mut pattern_map: [Option<&str>; 10] = [None; 10];

    // First pass: Identify digits with unique number of segments
    for &pattern in &patterns {
        match pattern.len() {
            2 => pattern_map[1] = Some(pattern),
            3 => pattern_map[7] = Some(pattern),
            4 => pattern_map[4] = Some(pattern),
            7 => pattern_map[8] = Some(pattern),
            _ => (),
        }
        // once 
        if pattern_map.iter().all(|&x| x.is_some()) {
            break;
        }
    }

    // Second pass: Deduce the rest based on unique characteristics
    for &pattern in &patterns {
        match pattern.len() {
            5 => { // Could be 2, 3, or 5
                if contains_all_chars(pattern, pattern_map[1].unwrap()) {
                    pattern_map[3] = Some(pattern);
                } else if intersection_count(pattern, pattern_map[4].unwrap()) == 2 {
                    pattern_map[2] = Some(pattern);
                } else {
                    pattern_map[5] = Some(pattern);
                }
            },
            6 => { // Could be 0, 6, or 9
                if !contains_all_chars(pattern, pattern_map[1].unwrap()) {
                    pattern_map[6] = Some(pattern);
                } else if contains_all_chars(pattern, pattern_map[4].unwrap()) {
                    pattern_map[9] = Some(pattern);
                } else {
                    pattern_map[0] = Some(pattern);
                }
            },
            _ => (),
        }
    }


    /*
    Decode the output values for example using the first text input line:
        
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe

    Output values are: ["fdgacbe", "cefdb", "cefbgd", "gcbe"]. 
    fold starts with an initial accumulator value of 0. 
    For the first output value "fdgacbe", pattern_map.iter().position(|&p| is_same_pattern(p.unwrap(), output)).unwrap() 
    finds the position of the pattern in pattern_map that matches "fdgacbe". Let's say this position is 3 so 
    10 * acc + 3 calculates the new accumulator value. Since acc is 0, the result is 3.

    The process repeats for the remaining output values. For example, for the second output value "cefdb", the position might be 1.
    The new accumulator value would be 13. Once all output values have been processed.
    
    Fold returns the final accumulator value, which is the decoded output value: 968175
    */
    outputs.iter().fold(0, |acc, &output| {

        10 * acc + pattern_map.iter().position(|&p| is_same_pattern(p.unwrap(), output)).unwrap() as u32
    })
}

/// Checks if `container` contains all characters of `contained`.
/// Returns true if all characters in `contained` are present in `container`, false otherwise.
fn contains_all_chars(container: &str, contained: &str) -> bool {
    contained.chars().all(|c| container.contains(c))
}

/// Counts the number of characters in the intersection of two patterns.
/// Returns the count of characters that appear in both `a` and `b`.
fn intersection_count(a: &str, b: &str) -> usize {
    a.chars().filter(|&c| b.contains(c)).count()
}

/// Checks if two patterns are the same (ignoring order).
/// Returns true if `a` and `b` have the same length and contain the same characters, false otherwise.
fn is_same_pattern(a: &str, b: &str) -> bool {
    a.len() == b.len() && contains_all_chars(a, b)
}