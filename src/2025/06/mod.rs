use crate::Solution;

pub struct Day06;

impl Solution for Day06 {
    fn solve(&self, input: &str) -> (String, String) {
        let input_lines: Vec<&str> = input.lines().filter(|x| !x.is_empty()).collect();
        let mut grid: Vec<Vec<u64>> = Vec::new();
        let mut operations: Vec<String> = Vec::new();

        for line in &input_lines {
            if line.contains('*') {
                let parts: Vec<&str> = line.split_whitespace().collect();
                for part in parts {
                    if part == "+" || part == "*" {
                        operations.push(part.to_string());
                    }
                }
            } else {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let mut problem: Vec<u64> = Vec::new();
                for part in parts {
                    if part.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                        if let Ok(num) = part.parse::<u64>() {
                            problem.push(num);
                        }
                    }
                }
                grid.push(problem);
            }
        }

        // Transpose grid: problems[i] contains all numbers for problem i
        let num_problems = if grid.is_empty() || operations.is_empty() {
            0
        } else {
            grid[0].len()
        };
        let mut problems: Vec<Vec<u64>> = vec![Vec::new(); num_problems];
        for i in 0..num_problems {
            for j in 0..grid.len() {
                problems[i].push(grid[j][i]);
            }
        }

        let mut total_p1 = 0u64;
        for i in 0..problems.len().min(operations.len()) {
            if operations[i] == "+" {
                let mut total = 0u64;
                for j in 0..problems[i].len() {
                    total += problems[i][j];
                }
                total_p1 += total;
            } else {
                let mut total = 1u64;
                for j in 0..problems[i].len() {
                    total *= problems[i][j];
                }
                total_p1 += total;
            }
        }

        let mut total_p2 = 0u64;
        let input_bytes: Vec<&[u8]> = input_lines.iter().map(|x| x.as_bytes()).collect();
        let max_len = input_bytes.iter().map(|line| line.len()).max().unwrap_or(0);
        let mut numbers: Vec<u64> = Vec::new();

        for i in (0..max_len).rev() {
            let mut is_all_empty = true;
            let mut number = 0u64;

            for j in 0..input_bytes.len() {
                if i >= input_bytes[j].len() {
                    continue;
                }

                let ch = input_bytes[j][i] as char;

                if ch != ' ' {
                    is_all_empty = false;
                }

                if ch.is_ascii_digit() {
                    number = number * 10 + (ch as u8 - b'0') as u64;
                }

                if j == input_bytes.len() - 1 && !is_all_empty {
                    numbers.push(number);
                    number = 0;
                }

                if ch == '+' || ch == '*' {
                    if ch == '+' {
                        let mut total = 0u64;
                        for k in 0..numbers.len() {
                            total += numbers[k];
                        }
                        total_p2 += total;
                    } else {
                        let mut total = 1u64;
                        for k in 0..numbers.len() {
                            total *= numbers[k];
                        }
                        total_p2 += total;
                    }
                    numbers.clear();
                }
            }
        }

        (total_p1.to_string(), total_p2.to_string())
    }

    fn examples(&self) -> Vec<(String, (String, String))> {
        vec![(
            "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ".to_string(),
            ("4277556".to_string(), "3263827".to_string()),
        )]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_examples;

    #[test]
    fn test_examples() {
        let solution = Day06;
        run_examples(&solution);
    }

    #[test]
    fn test_part1_given() {
        let solution = Day06;
        let given = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        let (p1, _) = solution.solve(given);
        assert_eq!(p1, "4277556");
    }

    #[test]
    fn test_part2_given() {
        let solution = Day06;
        let given = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let (_, p2) = solution.solve(given);
        assert_eq!(p2, "3263827");
    }

    #[test]
    fn test_part2_testcases() {
        let solution = Day06;
        let (_, p2) = solution.solve("64 \n23 \n314\n+  ");
        assert_eq!(p2, "1058");
    }
}
