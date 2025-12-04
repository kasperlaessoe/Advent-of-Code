use crate::Solution;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day24;

impl Solution for Day24 {
    fn solve(&self, input: &str) -> (String, String) {
        let mut weights: Vec<usize> = input
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() {
                    None
                } else {
                    line.parse().ok()
                }
            })
            .collect();

        weights.sort_by(|a, b| b.cmp(a));

        let part1 = find_best_group(&weights, 3).to_string();
        let part2 = find_best_group(&weights, 4).to_string();

        (part1, part2)
    }

    fn examples(&self) -> Vec<(String, (String, String))> {
        vec![(
            "1\n2\n3\n4\n5\n7\n8\n9\n10\n11".to_string(),
            ("99".to_string(), "44".to_string()),
        )]
    }
}

fn find_best_group(weights: &[usize], sections: usize) -> usize {
    let total: usize = weights.iter().sum();
    let target = total / sections;
    let mut cache = HashMap::new();

    for k in 1..=weights.len() {
        let mut best_qe = None;

        for combo in weights.iter().combinations(k) {
            let combo_values: Vec<usize> = combo.iter().map(|&&w| w).collect();
            let combo_sum: usize = combo_values.iter().sum();

            if combo_sum == target {
                let used: HashSet<usize> = combo_values.iter().copied().collect();
                let remaining: Vec<usize> = weights
                    .iter()
                    .filter(|&&w| !used.contains(&w))
                    .copied()
                    .collect();

                if can_split_into_groups_memoized(&remaining, target, sections - 1, &mut cache) {
                    let qe = combo_values.iter().product::<usize>();
                    if best_qe.is_none() || qe < best_qe.unwrap() {
                        best_qe = Some(qe);
                    }
                }
            }
        }

        if let Some(qe) = best_qe {
            return qe;
        }
    }

    0
}

fn can_split_into_groups_memoized(
    packages: &[usize],
    target_weight: usize,
    num_groups: usize,
    cache: &mut HashMap<(Vec<usize>, usize), bool>,
) -> bool {
    if num_groups == 1 {
        return packages.iter().sum::<usize>() == target_weight;
    }

    let mut key_packages = packages.to_vec();
    key_packages.sort();
    let key = (key_packages, num_groups);

    if let Some(&result) = cache.get(&key) {
        return result;
    }

    let result = {
        let mut found = false;
        for k in 1..=packages.len() {
            for combo in packages.iter().combinations(k) {
                let combo_values: Vec<usize> = combo.iter().map(|&&w| w).collect();
                let combo_sum: usize = combo_values.iter().sum();

                if combo_sum == target_weight {
                    let combo_set: HashSet<usize> = combo_values.iter().copied().collect();
                    let remaining: Vec<usize> = packages
                        .iter()
                        .filter(|&&w| !combo_set.contains(&w))
                        .copied()
                        .collect();

                    if can_split_into_groups_memoized(
                        &remaining,
                        target_weight,
                        num_groups - 1,
                        cache,
                    ) {
                        found = true;
                        break;
                    }
                }
            }
            if found {
                break;
            }
        }
        found
    };

    cache.insert(key, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_examples;

    #[test]
    fn test_examples() {
        let solution = Day24;
        run_examples(&solution);
    }
}
