use crate::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn solve(&self, input: &str) -> (String, String) {
        // TODO: Implement Part 1
        let part1 = "0".to_string();
        
        // TODO: Implement Part 2
        let part2 = "0".to_string();
        
        (part1, part2)
    }
    
    fn examples(&self) -> Vec<(String, (String, String))> {
        vec![
            // TODO: Add example test cases from problem description
            // Example: ("input".to_string(), ("expected_p1".to_string(), "expected_p2".to_string())),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_examples;
    
    #[test]
    fn test_examples() {
        let solution = Day01;
        run_examples(&solution);
    }
}
