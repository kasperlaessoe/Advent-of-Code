#[path = "01/mod.rs"]
mod day01;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let (p1, p2) = day01::solve();
    let elapsed = start.elapsed();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Time: {:?}", elapsed);
}
