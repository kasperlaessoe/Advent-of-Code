use std::fs;

pub fn solve() -> (u32, u32) {
    let input = fs::read_to_string("src/01/input.txt").unwrap();
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
                for _ in 0..dist {
                    pos = (pos - 1 + 100) % 100;
                    if pos == 0 {
                        p2 += 1;
                    }
                }
            }
            'R' => {
                for _ in 0..dist {
                    pos = (pos + 1) % 100;
                    if pos == 0 {
                        p2 += 1;
                    }
                }
            }
            _ => unreachable!(),
        }

        if pos == 0 {
            p1 += 1;
        }
    }

    (p1, p2)
}
