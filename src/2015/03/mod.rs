use crate::Solution;
use std::collections::HashSet;

pub struct Day03;

impl Solution for Day03 {
    fn solve(&self, input: &str) -> (String, String) {
        let instructions = input.trim();

        let mut visited = HashSet::new();
        let mut x = 0i32;
        let mut y = 0i32;
        visited.insert((x, y));

        for ch in instructions.chars() {
            match ch {
                '^' => y += 1,
                'v' => y -= 1,
                '>' => x += 1,
                '<' => x -= 1,
                _ => continue,
            }
            visited.insert((x, y));
        }
        let part1 = visited.len().to_string();

        let mut all_visited = HashSet::new();
        let mut santa_x = 0i32;
        let mut santa_y = 0i32;
        let mut robo_x = 0i32;
        let mut robo_y = 0i32;

        all_visited.insert((0, 0));

        for (i, ch) in instructions.chars().enumerate() {
            let (x, y) = if i % 2 == 0 {
                (&mut santa_x, &mut santa_y)
            } else {
                (&mut robo_x, &mut robo_y)
            };

            match ch {
                '^' => *y += 1,
                'v' => *y -= 1,
                '>' => *x += 1,
                '<' => *x -= 1,
                _ => continue,
            }

            all_visited.insert((*x, *y));
        }

        let part2 = all_visited.len().to_string();

        (part1, part2)
    }

    fn examples(&self) -> Vec<(String, (String, String))> {
        vec![
            (">".to_string(), ("2".to_string(), "2".to_string())),
            ("^>v<".to_string(), ("4".to_string(), "3".to_string())),
            (
                "^v^v^v^v^v".to_string(),
                ("2".to_string(), "11".to_string()),
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_examples;

    #[test]
    fn test_examples() {
        let solution = Day03;
        run_examples(&solution);
    }
}
