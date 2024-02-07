use std::collections::VecDeque;

struct Bracket {
    open_bracket_stack: VecDeque<char>,
    bracket_pairs: Vec<(char, char)>,
}

impl Bracket {
    fn new() -> Bracket {
        Bracket {
            open_bracket_stack: VecDeque::new(),
            bracket_pairs: vec![('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')],
        }
    }

    fn validate_syntax(&mut self, brackets: &str) -> Result<(), String> {
        for bracket in brackets.chars() {
            self.check_bracket(bracket)?;
        }
        Ok(())
    }

    fn check_bracket(&mut self, bracket: char) -> Result<(), String> {
        match bracket {
            '(' | '[' | '{' | '<' => {
                self.handle_open_bracket(bracket);
                Ok(())
            }
            ')' | ']' | '}' | '>' => self.handle_closed_bracket(bracket),
            _ => Err(format!("Invalid character: {}", bracket)),
        }
    }

    fn handle_open_bracket(&mut self, bracket: char) {
        self.open_bracket_stack.push_back(bracket);
    }

    fn handle_closed_bracket(&mut self, closed_bracket: char) -> Result<(), String> {
        if let Some(last_open) = self.open_bracket_stack.pop_back() {
            if let Some((_, expected_close)) = self
                .bracket_pairs
                .iter()
                .find(|&(open, _)| *open == last_open)
            {
                if closed_bracket == *expected_close {
                    return Ok(());
                } else {
                    return Err(format!(
                        "Expected {}, but found {} instead.",
                        expected_close, closed_bracket
                    ));
                }
            }
        }
        Err(String::from("Unexpected closing bracket"))
    }
}

fn main() {
    let input_brackets = String::from("[{[{({}]{}}([{[{{{}}([]");
    let mut bracket = Bracket::new();
    let check_brackets = bracket.validate_syntax(&input_brackets);
    println!("Syntax check: {:?}", check_brackets);
    println!("Bracket pairs: {:?}", bracket.bracket_pairs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corrupted_lines() {
        let mut bracket = Bracket::new();
        let examples = vec![
            "{([(<{}[<>[]}>{[]{[(<()>",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
        ];

        let expected_errors = vec![
            "Expected ], but found } instead.",
            "Expected ], but found ) instead.",
            "Expected ), but found ] instead.",
            "Expected >, but found ) instead.",
            "Expected ], but found > instead.",
        ];

        for (example, expected_error) in examples.into_iter().zip(expected_errors.into_iter()) {
            let result = bracket.validate_syntax(&example);
            assert_eq!(result.unwrap_err().as_str(), expected_error);
        }
    }
}
