use crate::Solution;
use std::collections::HashMap;

/// Represents a discovered solution with its year and day
pub struct SolutionInfo {
    pub year: u16,
    pub day: u8,
    pub solution: Box<dyn Solution>,
}

/// Registry of all discovered solutions
pub struct SolutionRegistry {
    solutions: HashMap<(u16, u8), Box<dyn Solution>>,
}

impl SolutionRegistry {
    pub fn new() -> Self {
        Self {
            solutions: HashMap::new(),
        }
    }

    /// Register a solution for a specific year and day
    pub fn register<S: Solution + 'static>(&mut self, year: u16, day: u8, solution: S) {
        self.solutions.insert((year, day), Box::new(solution));
    }

    /// Get a solution for a specific year and day
    pub fn get(&self, year: u16, day: u8) -> Option<&dyn Solution> {
        self.solutions.get(&(year, day)).map(|s| s.as_ref())
    }

    /// Get all registered solutions
    pub fn all(&self) -> Vec<(u16, u8)> {
        let mut result: Vec<_> = self.solutions.keys().copied().collect();
        result.sort();
        result
    }

    /// Get all solutions for a specific year
    pub fn for_year(&self, year: u16) -> Vec<u8> {
        let mut result: Vec<u8> = self
            .solutions
            .keys()
            .filter(|(y, _)| *y == year)
            .map(|(_, d)| *d)
            .collect();
        result.sort();
        result
    }

    /// Get all unique years
    pub fn years(&self) -> Vec<u16> {
        let mut years: Vec<u16> = self.solutions.keys().map(|(y, _)| *y).collect();
        years.sort();
        years.dedup();
        years
    }
}

/// Build the solution registry by registering all available solutions
pub fn build_registry() -> SolutionRegistry {
    let mut registry = SolutionRegistry::new();

    // Register all solutions
        registry.register(2020, 1, crate::year2020::day01::Day01);
        registry.register(2025, 6, crate::year2025::day06::Day06);
        registry.register(2025, 5, crate::year2025::day05::Day05);
        registry.register(2015, 24, crate::year2015::day24::Day24);
        registry.register(2015, 3, crate::year2015::day03::Day03);
        registry.register(2025, 4, crate::year2025::day04::Day04);
        registry.register(2015, 2, crate::year2015::day02::Day02);
        registry.register(2025, 2, crate::year2025::day02::Day02);
    registry.register(2015, 1, crate::year2015::day01::Day01);
    registry.register(2025, 1, crate::year2025::day01::Day01);
    registry.register(2025, 3, crate::year2025::day03::Day03);

    registry
}
