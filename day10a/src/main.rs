use std::collections::HashMap;

struct Bracket {
    bracket_vec: Vec<String>,
    open_brackets: Vec<String>,
    closed_brackets: HashMap<String, String>,
}

impl Bracket {
    fn new() -> Bracket {
        Bracket {
            bracket_vec: vec![],
            open_brackets: vec!["(".to_string(), "[".to_string(), "{".to_string(), "<".to_string()],
            closed_brackets: [
                (")".to_string(), "(".to_string()),
                ("]".to_string(), "[".to_string()),
                ("}".to_string(), "{".to_string()),
                (">".to_string(), "<".to_string()),
            ].iter().cloned().collect(),
        }
    }
    
    fn check_bracket(&mut self, bracket: &str) -> Result<(), String> {
        if self.open_brackets.contains(&bracket.to_string()) {
            self.bracket_vec.push(bracket.to_string());
            Ok(())
        } else if let Some(matching_bracket) = self.closed_brackets.get(bracket) {
            if self.bracket_vec.last() == Some(matching_bracket) {
                self.bracket_vec.pop();
                Ok(())
            } else {
                Err(format!("Expected {}, but found {} instead.", matching_bracket, bracket))
            }
        } else {
            Err(format!("Unexpected bracket: {}", bracket))
        }
    }

    fn 
}

fn main() {
    let mut bracket = Bracket::new();
    println!("Open brackets: {:?}", bracket.open_brackets);
    println!("Closed brackets: {:?}", bracket.closed_brackets);
    match bracket.check_bracket("(") {
        Ok(_) => println!("Bracket checked successfully: {:?}", bracket.bracket_vec),
        Err(e) => println!("Error checking bracket: {}", e),
    }
}