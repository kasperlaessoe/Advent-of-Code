use crate::Solution;
use std::collections::{HashSet, VecDeque};

pub struct Day04;

impl Solution for Day04 {
    fn solve(&self, input: &str) -> (String, String) {
        let grid: Vec<Vec<char>> = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.chars().collect())
            .collect();

        if grid.is_empty() {
            return ("0".to_string(), "0".to_string());
        }

        let rows = grid.len();
        let cols = grid[0].len();

        let mut accessible_count = 0;
        let mut working_grid = grid;
        let mut queue = VecDeque::new();

        for i in 0..rows {
            for j in 0..cols {
                if working_grid[i][j] == '@' {
                    let adjacent_rolls = count_adjacent_rolls(&working_grid, i, j, rows, cols);
                    if adjacent_rolls < 4 {
                        accessible_count += 1;
                        queue.push_back((i, j));
                    }
                }
            }
        }

        let part1 = accessible_count.to_string();

        let mut processed = HashSet::new();

        let mut total_removed = 0;

        while let Some((i, j)) = queue.pop_front() {
            if processed.contains(&(i, j)) || working_grid[i][j] != '@' {
                continue;
            }

            let adjacent_rolls = count_adjacent_rolls(&working_grid, i, j, rows, cols);

            if adjacent_rolls < 4 {
                working_grid[i][j] = '.';
                processed.insert((i, j));
                total_removed += 1;

                let has_top = i > 0;
                let has_bottom = i + 1 < rows;
                let has_left = j > 0;
                let has_right = j + 1 < cols;

                if has_top {
                    if has_left
                        && working_grid[i - 1][j - 1] == '@'
                        && !processed.contains(&(i - 1, j - 1))
                    {
                        queue.push_back((i - 1, j - 1));
                    }
                    if working_grid[i - 1][j] == '@' && !processed.contains(&(i - 1, j)) {
                        queue.push_back((i - 1, j));
                    }
                    if has_right
                        && working_grid[i - 1][j + 1] == '@'
                        && !processed.contains(&(i - 1, j + 1))
                    {
                        queue.push_back((i - 1, j + 1));
                    }
                }
                if has_left && working_grid[i][j - 1] == '@' && !processed.contains(&(i, j - 1)) {
                    queue.push_back((i, j - 1));
                }
                if has_right && working_grid[i][j + 1] == '@' && !processed.contains(&(i, j + 1)) {
                    queue.push_back((i, j + 1));
                }
                if has_bottom {
                    if has_left
                        && working_grid[i + 1][j - 1] == '@'
                        && !processed.contains(&(i + 1, j - 1))
                    {
                        queue.push_back((i + 1, j - 1));
                    }
                    if working_grid[i + 1][j] == '@' && !processed.contains(&(i + 1, j)) {
                        queue.push_back((i + 1, j));
                    }
                    if has_right
                        && working_grid[i + 1][j + 1] == '@'
                        && !processed.contains(&(i + 1, j + 1))
                    {
                        queue.push_back((i + 1, j + 1));
                    }
                }
            }
        }

        let part2 = total_removed.to_string();

        (part1, part2)
    }

    fn examples(&self) -> Vec<(String, (String, String))> {
        vec![(
            "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.".to_string(),
            ("13".to_string(), "43".to_string()),
        )]
    }
}

fn count_adjacent_rolls(grid: &[Vec<char>], i: usize, j: usize, rows: usize, cols: usize) -> usize {
    let mut adjacent_rolls = 0;
    let has_top = i > 0;
    let has_bottom = i + 1 < rows;
    let has_left = j > 0;
    let has_right = j + 1 < cols;

    if has_top {
        if has_left && grid[i - 1][j - 1] == '@' {
            adjacent_rolls += 1;
        }
        if grid[i - 1][j] == '@' {
            adjacent_rolls += 1;
        }
        if has_right && grid[i - 1][j + 1] == '@' {
            adjacent_rolls += 1;
        }
    }
    if has_left && grid[i][j - 1] == '@' {
        adjacent_rolls += 1;
    }
    if has_right && grid[i][j + 1] == '@' {
        adjacent_rolls += 1;
    }
    if has_bottom {
        if has_left && grid[i + 1][j - 1] == '@' {
            adjacent_rolls += 1;
        }
        if grid[i + 1][j] == '@' {
            adjacent_rolls += 1;
        }
        if has_right && grid[i + 1][j + 1] == '@' {
            adjacent_rolls += 1;
        }
    }

    adjacent_rolls
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_examples;

    #[test]
    fn test_examples() {
        let solution = Day04;
        run_examples(&solution);
    }
}
