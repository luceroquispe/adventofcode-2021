// Define the Bracket enum with variants for each type of bracket
#[derive(PartialEq, Clone, Copy)]
enum Bracket {
    Round,  // ()
    Square, // []
    Curly,  // {}
    Angle,  // <>
}

struct ParserState {
    stack: Vec<Bracket>,
}

impl ParserState {
    fn new() -> Self {
        Self { stack: vec![] }
    }

    // Push an opening bracket onto the stack
    fn push(&mut self, bracket: Bracket) {
        self.stack.push(bracket);
    }

    fn pop(&mut self, closing: Bracket) -> Option<Bracket> {
        match self.stack.pop() {
            Some(opening) if opening == closing => Some(opening),
            _ => None,
        }
    }

    fn completion_string(&self) -> String {
        let mut result = String::new();
        for &bracket in self.stack.iter().rev() {
            result.push(match bracket {
                Bracket::Round => ')',
                Bracket::Square => ']',
                Bracket::Curly => '}',
                Bracket::Angle => '>',
            });
        }
        result
    }
}

fn calculate_score(completion: &str) -> i64 {
    let mut score = 0;
    for ch in completion.chars() {
        score *= 5;
        score += match ch {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Unexpected character in completion string"),
        };
    }
    score
}

fn handle_opening_bracket(state: &mut ParserState, ch: char) {
    match ch {
        '(' => state.push(Bracket::Round),
        '[' => state.push(Bracket::Square),
        '{' => state.push(Bracket::Curly),
        '<' => state.push(Bracket::Angle),
        _ => (),
    }
}
fn handle_closing_bracket(state: &mut ParserState, ch: char) -> bool {
    // assume its fine for now
    let mut is_corrupt: bool = false;
    match ch {
        ')' | ']' | '}' | '>' => {
            if state
                .pop(match ch {
                    ')' => Bracket::Round,
                    ']' => Bracket::Square,
                    '}' => Bracket::Curly,
                    '>' => Bracket::Angle,
                    _ => panic!("Unexpected closing bracket"),
                })
                .is_none()
            {
                is_corrupt = true;
            }
        }
        _ => (),
    }
    is_corrupt
}

fn find_median_score(scores: &mut Vec<i64>) -> i64 {
    scores.sort();
    let middle_index = scores.len() / 2;
    scores[middle_index]
}

// Main function to parse lines and calculate scores
fn main() {
    let mut scores = Vec::new();
    let lines: Vec<&str> = include_str!("../data/data.txt").lines().collect();

    for line in lines {
        let mut state = ParserState::new();
        for ch in line.chars() {
            if handle_closing_bracket(&mut state, ch) {
                break; // Corrupted line, stop parsing
            } else {
                handle_opening_bracket(&mut state, ch);
            }
        }
        let completion = state.completion_string();
        let score = calculate_score(&completion);
        scores.push(score);
    }

    // Sort the scores and find the middle score
    scores.sort();
    println!("{:?}", &scores);
    let middle_score = find_median_score(&mut scores);

    println!("Middle score: {}", middle_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_state_has_empty_completion_string() {
        // Arrange, Act
        let mut state = ParserState::new();
        handle_opening_bracket(&mut state, '(');
        handle_closing_bracket(&mut state, ')');
        assert!(state.completion_string().is_empty())
    }

    #[test]
    fn test_parser_state_returns_completion_string() {
        // Arrange, Act
        let mut state = ParserState::new();
        handle_opening_bracket(&mut state, '(');
        handle_opening_bracket(&mut state, '(');
        handle_closing_bracket(&mut state, ')');
        // Finds stack has "(" then returns matching completion
        assert_eq!(state.completion_string(), ")");
    }

    #[test]
    fn test_handle_closing_bracket_matching_ok() {
        // Arrange
        let mut state = ParserState::new();
        state.push(Bracket::Round); // Opening bracket
        state.push(Bracket::Square); // Another opening bracket
        assert_eq!(handle_closing_bracket(&mut state, ']'), false); // Correct closing bracket
        assert_eq!(handle_closing_bracket(&mut state, ')'), false); // Correct closing bracket
    }

    #[test]
    fn test_calculate_score_empty_string() {
        assert_eq!(calculate_score(""), 0);
    }

    #[test]
    fn test_calculate_score_single_character() {
        assert_eq!(calculate_score(")"), 1);
        assert_eq!(calculate_score("]"), 2);
        assert_eq!(calculate_score("}"), 3);
        assert_eq!(calculate_score(">"), 4);
    }

    #[test]
    fn test_calculate_score_multiple_characters() {
        assert_eq!(calculate_score(")>"), 9); //  5 * ((5 * 0) + 1) + 4 = 9
        assert_eq!(calculate_score("]})"), 66); //  5 * (5 * ((5 * 0) + 2) + 3) + 1 = 66
        assert_eq!(calculate_score("}>"), 19); //  5 * ((5 * 0) + 3) + 4 = 19
        assert_eq!(calculate_score("}}>}>))))"), 1480781);
        assert_eq!(calculate_score("])}>"), 294);
    }

    #[test]
    fn test_find_median_score() {
        let mut scores = vec![294, 5566, 288957, 995444, 1480781];
        let median = find_median_score(&mut scores);
        assert_eq!(median, 288957);
    }
}
