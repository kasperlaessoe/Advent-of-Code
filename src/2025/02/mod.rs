use crate::Solution;

pub struct Day02;

impl Solution for Day02 {
    fn solve(&self, input: &str) -> (String, String) {
        let input = input.trim();
        let mut sum_p1 = 0u64;
        let mut sum_p2 = 0u64;

        // Parse ranges
        for range_str in input.split(',') {
            let range_str = range_str.trim();
            if range_str.is_empty() {
                continue;
            }

            let parts: Vec<&str> = range_str.split('-').collect();
            if parts.len() != 2 {
                continue;
            }

            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();

            for id in generate_invalid_part1(start, end) {
                sum_p1 += id;
            }
            for id in generate_invalid_part2(start, end) {
                sum_p2 += id;
            }
        }

        (sum_p1.to_string(), sum_p2.to_string())
    }

    fn examples(&self) -> Vec<(String, (String, String))> {
        vec![(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string(),
            ("1227775554".to_string(), "4174379265".to_string()),
        )]
    }
}

fn generate_invalid_part1(start: u64, end: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let start_digits = number_to_digits(start);
    let end_digits = number_to_digits(end);
    let min_len = start_digits.len();
    let max_len = end_digits.len();

    for len in min_len..=max_len {
        if len % 2 != 0 {
            continue;
        }

        let half = len / 2;
        let min_half = if len == min_len {
            digits_to_number(&start_digits[..half.min(start_digits.len())])
        } else {
            10u64.pow((half - 1) as u32)
        };
        let max_half = if len == max_len {
            digits_to_number(&end_digits[..half.min(end_digits.len())])
        } else {
            10u64.pow(half as u32) - 1
        };

        for half_num in min_half..=max_half {
            let half_digits = number_to_digits(half_num);
            if half_digits.len() != half {
                continue;
            }
            let full_digits: Vec<u8> = half_digits
                .iter()
                .chain(half_digits.iter())
                .copied()
                .collect();
            let full_num = digits_to_number(&full_digits);

            if full_num >= start && full_num <= end {
                result.push(full_num);
            } else if full_num > end {
                break;
            }
        }
    }

    result
}

fn generate_invalid_part2(start: u64, end: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let start_digits = number_to_digits(start);
    let end_digits = number_to_digits(end);
    let min_len = start_digits.len();
    let max_len = end_digits.len();

    for len in min_len..=max_len {
        for pattern_len in 1..=len / 2 {
            if len % pattern_len != 0 {
                continue;
            }

            let repetitions = len / pattern_len;
            if repetitions < 2 {
                continue;
            }

            let min_pattern = if len == min_len && pattern_len <= start_digits.len() {
                digits_to_number(&start_digits[..pattern_len])
            } else {
                10u64.pow((pattern_len - 1) as u32)
            };
            let max_pattern = if len == max_len && pattern_len <= end_digits.len() {
                digits_to_number(&end_digits[..pattern_len])
            } else {
                10u64.pow(pattern_len as u32) - 1
            };

            for pattern_num in min_pattern..=max_pattern {
                let pattern_digits = number_to_digits(pattern_num);
                if pattern_digits.len() != pattern_len {
                    continue;
                }

                let full_digits: Vec<u8> = (0..repetitions)
                    .flat_map(|_| pattern_digits.iter().copied())
                    .collect();
                let full_num = digits_to_number(&full_digits);

                if full_num >= start && full_num <= end {
                    result.push(full_num);
                } else if full_num > end {
                    break;
                }
            }
        }
    }

    result.sort();
    result.dedup();
    result
}

fn number_to_digits(mut n: u64) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }

    let mut digits = Vec::new();
    while n > 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }
    digits.reverse();
    digits
}

fn digits_to_number(digits: &[u8]) -> u64 {
    digits.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_examples;

    #[test]
    fn test_examples() {
        let solution = Day02;
        run_examples(&solution);
    }
}
