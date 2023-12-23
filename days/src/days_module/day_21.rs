use crate::days_module::day::Day;
use helpers::cube::cube::Cube;
use std::cmp::{min, Ordering};
use std::collections::HashSet;
use std::str::FromStr;

pub struct Day21 {}

impl Day for Day21 {
    fn get_id(&self) -> String {
        "day_21".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        "".to_string()
    }

    fn part_b(&self, input: &String) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day21 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day21 {}.test_day_part(&'b')
    }
}
