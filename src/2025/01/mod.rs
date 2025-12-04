use crate::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn solve(&self, input: &str) -> (String, String) {
        let mut pos = 50i32;
        let mut p1 = 0;
        let mut p2 = 0;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let dir = line.chars().next().unwrap();
            let dist: i32 = line[1..].parse().unwrap();

            match dir {
                'L' => {
                    p2 += (pos + dist - 1) / 100;
                    pos = ((pos - dist) % 100 + 100) % 100;
                }
                'R' => {
                    p2 += (pos + dist) / 100;
                    pos = (pos + dist) % 100;
                }
                _ => unreachable!(),
            }

            if pos == 0 {
                p1 += 1;
            }
        }

        (p1.to_string(), p2.to_string())
    }

    fn examples(&self) -> Vec<(String, (String, String))> {
        vec![(
            "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string(),
            ("3".to_string(), "6".to_string()),
        )]
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
