use crate::Solution;

pub struct Day02;

impl Solution for Day02 {
    fn solve(&self, input: &str) -> (String, String) {
        let mut total_paper = 0u32;
        let mut total_ribbon = 0u32;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let dimensions: Vec<u32> = line.split('x').map(|s| s.parse().unwrap()).collect();

            if dimensions.len() != 3 {
                continue;
            }

            let l = dimensions[0];
            let w = dimensions[1];
            let h = dimensions[2];

            let surface_area = 2 * l * w + 2 * w * h + 2 * h * l;

            let side1 = l * w;
            let side2 = w * h;
            let side3 = h * l;
            let smallest_side = side1.min(side2).min(side3);

            total_paper += surface_area + smallest_side;

            let mut sides = vec![l, w, h];
            sides.sort();
            let perimeter = 2 * sides[0] + 2 * sides[1];
            let volume = l * w * h;

            total_ribbon += perimeter + volume;
        }

        (total_paper.to_string(), total_ribbon.to_string())
    }

    fn examples(&self) -> Vec<(String, (String, String))> {
        vec![
            ("2x3x4".to_string(), ("58".to_string(), "34".to_string())),
            ("1x1x10".to_string(), ("43".to_string(), "14".to_string())),
            (
                "2x3x4\n1x1x10".to_string(),
                ("101".to_string(), "48".to_string()),
            ),
        ]
    }
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
