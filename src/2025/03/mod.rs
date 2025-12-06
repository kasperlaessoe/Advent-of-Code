use crate::Solution;

pub struct Day03;

impl Solution for Day03 {
    fn solve(&self, input: &str) -> (String, String) {
        let mut total_p1 = 0u64;
        let mut total_p2 = 0u64;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

            total_p1 += find_max_k_digits(&digits, 2);
            total_p2 += find_max_k_digits(&digits, 12);
        }

        (total_p1.to_string(), total_p2.to_string())
    }

    fn examples(&self) -> Vec<(String, (String, String))> {
        vec![
            (
                "987654321111111".to_string(),
                ("98".to_string(), "987654321111".to_string()),
            ),
            (
                "811111111111119".to_string(),
                ("89".to_string(), "811111111119".to_string()),
            ),
            (
                "234234234234278".to_string(),
                ("78".to_string(), "434234234278".to_string()),
            ),
            (
                "818181911112111".to_string(),
                ("92".to_string(), "888911112111".to_string()),
            ),
            (
                "987654321111111\n811111111111119\n234234234234278\n818181911112111".to_string(),
                ("357".to_string(), "3121910778619".to_string()),
            ),
        ]
    }
}

fn find_max_k_digits(digits: &[u32], k: usize) -> u64 {
    if k == 0 {
        return 0;
    }
    if k > digits.len() {
        return 0;
    }
    if k == digits.len() {
        return digits.iter().fold(0u64, |acc, &d| acc * 10 + d as u64);
    }

    let mut result = 0u64;
    let mut start = 0;

    for pos in 0..k {
        let end = digits.len() - (k - pos);
        let mut max_digit = 0u32;
        let mut max_pos = start;

        for i in start..=end {
            if digits[i] > max_digit {
                max_digit = digits[i];
                max_pos = i;
            }
        }

        result = result * 10 + max_digit as u64;
        start = max_pos + 1;
    }

    result
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
