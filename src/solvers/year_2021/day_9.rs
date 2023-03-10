use crate::solvers::{Solver, SolverResult};

pub fn create() -> Day9 {
    let input = include_str!("inputs/09.txt");
    let height_map = HeightMap::from_input(input);

    Day9 { height_map }
}

pub struct Day9 {
    height_map: HeightMap,
}

impl Solver for Day9 {
    fn run_part1(&self) -> SolverResult {
        let mut sum = 0;

        for y in 0..self.height_map.height {
            for x in 0..self.height_map.width {
                let low_point = self.height_map.get_low_point(x, y);
                if let Option::Some(value) = low_point {
                    sum += value + 1;
                }
            }
        }

        sum.into()
    }

    fn run_part2(&self) -> SolverResult {
        let length = self.height_map.array.len();

        let mut top_basins: [i64; 3] = [0; 3];

        let mut visited: Vec<bool> = Vec::with_capacity(length);
        visited.resize(length, false);
        
        for y in 0..self.height_map.height {
            for x in 0..self.height_map.width {

                let index = self.height_map.get_index(x, y);
                if visited[index] || self.height_map.array[index] == 9 {
                    continue;
                }

                let mut basin_size = 0;
                self.height_map.explore_basin(x, y, &mut visited, &mut basin_size); 

                if basin_size > top_basins[0] {
                    top_basins[2] = top_basins[1];
                    top_basins[1] = top_basins[0];
                    top_basins[0] = basin_size;
                } else if basin_size > top_basins[1] {
                    top_basins[2] = top_basins[1];
                    top_basins[1] = basin_size;
                } else if basin_size > top_basins[2] {
                    top_basins[2] = basin_size;
                }
            }
        }

        (top_basins[0] * top_basins[1] * top_basins[2]).into()
    }
}

struct HeightMap {
    array: Vec<i64>,
    width: usize,
    height: usize,
}

impl HeightMap {
    fn from_input(input: &str) -> HeightMap {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let mut array = Vec::with_capacity(width * height);
        for line in input.lines() {
            for c in line.chars() {
                let value = (c as i64) - ('0' as i64);
                array.push(value);
            }
        }

        assert_eq!(array.len(), array.capacity());

        HeightMap {
            array,
            width,
            height
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width);
        assert!(y < self.height);
        x + y * self.width
    }

    fn get_low_point(&self, x: usize, y: usize) -> Option<i64> {
        let index = self.get_index(x, y);
        let value = self.array[index];

        if x > 0 {
            let other_index = self.get_index(x - 1, y);
            let other_value = self.array[other_index];
            if other_value <= value {
                return None;
            }
        }

        if x < self.width - 1 {
            let other_index = self.get_index(x + 1, y);
            let other_value = self.array[other_index];
            if other_value <= value {
                return None;
            }
        }

        if y > 0 {
            let other_index = self.get_index(x , y - 1);
            let other_value = self.array[other_index];
            if other_value <= value {
                return None;
            }
        }

        if y < self.height - 1 {
            let other_index = self.get_index(x, y + 1);
            let other_value = self.array[other_index];
            if other_value <= value {
                return None;
            }
        }

        Some(value)
    }

    fn explore_basin(&self, x: usize, y: usize, visited: &mut Vec<bool>, basin_size: &mut i64) {
        let index = self.get_index(x, y);
        if visited[index] || self.array[index] == 9 {
            return;
        }

        *basin_size += 1;
        visited[index] = true;

        if x > 0 { 
            self.explore_basin(x - 1, y, visited, basin_size); 
        } 

        if x < self.width - 1 { 
            self.explore_basin(x + 1, y, visited, basin_size); 
        }

        if y > 0 { 
            self.explore_basin(x, y - 1, visited, basin_size); 
        }

        if y < self.height - 1 { 
            self.explore_basin(x, y + 1, visited, basin_size); 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 439.into(), "Part1");
        assert_eq!(day.run_part2(), 900900.into(), "Part2");
    }
}