/// Trait that all Advent of Code day solutions must implement
pub trait Solution {
    /// Solve both parts of the puzzle given the input
    /// Returns (part1_result, part2_result) as strings
    fn solve(&self, input: &str) -> (String, String);

    /// Return example test cases from the problem description
    /// Each tuple is (input, (expected_part1, expected_part2))
    fn examples(&self) -> Vec<(String, (String, String))>;
}

/// Helper function to run example tests
#[cfg(test)]
pub fn run_examples<S: Solution>(solution: &S) {
    for (i, (input, (expected_p1, expected_p2))) in solution.examples().iter().enumerate() {
        let (actual_p1, actual_p2) = solution.solve(input);
        assert_eq!(
            actual_p1,
            *expected_p1,
            "Example {} Part 1 failed: expected {}, got {}",
            i + 1,
            expected_p1,
            actual_p1
        );
        assert_eq!(
            actual_p2,
            *expected_p2,
            "Example {} Part 2 failed: expected {}, got {}",
            i + 1,
            expected_p2,
            actual_p2
        );
    }
}

// Year modules
#[path = "year2015.rs"]
pub mod year2015;

#[path = "year2025.rs"]
pub mod year2025;
pub mod year2020;

pub mod discovery;
pub mod tui;
