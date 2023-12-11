use crate::days_module::day::Day;
use helpers::manhattan_distance;
use std::collections::HashSet;

fn solve(input: &String, multiplier: usize) -> usize {
    // Find empty rows and column indices.
    let mut line_length = 0;
    let mut x_hash = HashSet::new();
    let mut y_hash = HashSet::new();
    for (x, line) in input.split("\n").enumerate() {
        line_length = line.len();
        for (y, char_) in line.chars().enumerate() {
            if char_ == '#' {
                x_hash.insert(x);
                y_hash.insert(y);
            }
        }
    }

    let empty_x = (0..line_length)
        .collect::<HashSet<usize>>()
        .difference(&x_hash)
        .map(|v| v.clone())
        .collect::<HashSet<usize>>();
    let empty_y = (0..line_length)
        .collect::<HashSet<usize>>()
        .difference(&y_hash)
        .map(|v| v.clone())
        .collect::<HashSet<usize>>();

    // Find coordinates, shifted.
    let mut coordinates = Vec::new();
    for (x, line) in input.split("\n").enumerate() {
        line_length = line.len();
        for (y, char_) in line.chars().enumerate() {
            if char_ == '#' {
                let x_shift = empty_x.iter().filter(|x_empty| **x_empty < x).count() * (multiplier - 1);
                let y_shift = empty_y.iter().filter(|y_empty| **y_empty < y).count() * (multiplier - 1);
                coordinates.push((x + x_shift, y + y_shift));
            }
        }
    }

    // Sum every Manhattan distance.
    let mut result = 0;
    for first_coordinate in &coordinates {
        for second_coordinate in &coordinates {
            result += manhattan_distance(first_coordinate, second_coordinate);
        }
    }

    result / 2
}

pub struct Day11 {}

impl Day for Day11 {
    fn get_id(&self) -> String {
        "day_11".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        solve(input, 2).to_string()
    }

    fn part_b(&self, input: &String) -> String {
        solve(input, 1000000).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day11 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day11 {}.test_day_part(&'b')
    }
}
