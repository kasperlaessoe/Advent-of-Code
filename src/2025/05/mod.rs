use crate::Solution;

pub struct Day05;

impl Solution for Day05 {
    fn solve(&self, input: &str) -> (String, String) {
        // TODO: Implement Part 1
        let lines = input.lines().collect::<Vec<&str>>();
        // Parse input into ranges and ids
        let mut ranges = Vec::new();
        let mut ids = Vec::new();
        let mut parsing_ranges = true;

        for line in lines.iter().map(|l| l.trim()) {
            if parsing_ranges {
                if line.is_empty() {
                    parsing_ranges = false;
                } else if line.contains('-') {
                    // Parse as range
                    if let Some((start, end)) = line.split_once('-') {
                        if let (Ok(start), Ok(end)) = (start.parse::<u64>(), end.parse::<u64>()) {
                            ranges.push((start, end));
                        }
                    }
                }
            } else if !line.is_empty() {
                if let Ok(n) = line.parse::<u64>() {
                    ids.push(n);
                }
            }
        }
        // Count how many IDs are fresh (in at least one range)
        let mut count = 0;
        for id in ids.iter() {
            // Check if this ID is in any range
            let is_fresh = ranges
                .iter()
                .any(|(start, end)| *id >= *start && *id <= *end);
            if is_fresh {
                count += 1;
            }
        }
        let part1 = count.to_string();

        // Part 2: Count total unique fresh ingredient IDs across all ranges
        // Sort ranges by start value
        ranges.sort_by_key(|(start, _)| *start);

        // Merge overlapping/adjacent ranges
        let mut combined_ranges = Vec::new();
        for range in ranges {
            if combined_ranges.is_empty() {
                combined_ranges.push(range);
            } else {
                let last = combined_ranges.last_mut().unwrap();
                if range.0 <= last.1 + 1 {
                    // Overlapping or adjacent - extend the range
                    last.1 = last.1.max(range.1);
                } else {
                    // No overlap - add as new range
                    combined_ranges.push(range);
                }
            }
        }

        // Sum the lengths of all merged ranges
        let part2 = combined_ranges
            .iter()
            .map(|(start, end)| end - start + 1)
            .sum::<u64>()
            .to_string();

        (part1, part2)
    }

    fn examples(&self) -> Vec<(String, (String, String))> {
        vec![(
            "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32".to_string(),
            ("3".to_string(), "14".to_string()),
        )]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_examples;

    #[test]
    fn test_examples() {
        let solution = Day05;
        run_examples(&solution);
    }
}
