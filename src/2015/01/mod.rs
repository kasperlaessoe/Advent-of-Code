use crate::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn solve(&self, input: &str) -> (String, String) {
        let instructions = input.trim();

        let mut floor = 0i32;
        let mut first_basement_pos = None;

        for (i, ch) in instructions.chars().enumerate() {
            match ch {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => continue,
            }
            if first_basement_pos.is_none() && floor == -1 {
                first_basement_pos = Some(i + 1);
            }
        }

        (
            floor.to_string(),
            first_basement_pos.unwrap_or(0).to_string(),
        )
    }

    fn examples(&self) -> Vec<(String, (String, String))> {
        vec![
            ("(())".to_string(), ("0".to_string(), "0".to_string())),
            ("()()".to_string(), ("0".to_string(), "0".to_string())),
            ("(((".to_string(), ("3".to_string(), "0".to_string())),
            ("(()(()(".to_string(), ("3".to_string(), "0".to_string())),
            ("))(((((".to_string(), ("3".to_string(), "1".to_string())),
            ("())".to_string(), ("-1".to_string(), "3".to_string())),
            ("))(".to_string(), ("-1".to_string(), "1".to_string())),
            (")))".to_string(), ("-3".to_string(), "1".to_string())),
            (")())())".to_string(), ("-3".to_string(), "1".to_string())),
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
