use itertools::Itertools;
use crate::solvers::{Solver, SolverResult};

pub fn create() -> Day3 {
    let input = include_str!("inputs/03.txt");
    let rows: Vec<String> = input.lines().map_into().collect_vec();

    Day3 { rows }
}

pub struct Day3 {
    rows: Vec<String>
}

impl Solver for Day3 {
    fn run_part1(&self) -> SolverResult {
        self.compute_slope(3, 1).into()
    }

    fn run_part2(&self) -> SolverResult {
        let tree_count_0 = self.compute_slope(1, 1);
        let tree_count_1 = self.compute_slope(3, 1);
        let tree_count_2 = self.compute_slope(5, 1);
        let tree_count_3 = self.compute_slope(7, 1);
        let tree_count_4 = self.compute_slope(1, 2);
    
        (tree_count_0 * tree_count_1 * tree_count_2 * tree_count_3 * tree_count_4).into()
    }
}

impl Day3 {
    fn compute_slope(&self, slope_right: usize, slope_down: usize) -> i64 {
        if self.rows.is_empty() {
            println!("Invalid rows!");
            return  0;
        }
    
        let length = self.rows.len();
        let width = self.rows[0].chars().count();
        let mut down = 0;
        let mut right = 0;
        let mut tree_count = 0;
    
        while down < length{
            let a = self.rows[down].chars().nth(right).unwrap_or_default();
            if a == '#' {
                tree_count += 1;
            }
    
            down += slope_down;
            right += slope_right;
            right %= width;
        }
    
        tree_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 289.into(), "Part1");
        assert_eq!(day.run_part2(), 5522401584_i64.into(), "Part2");
    }
}