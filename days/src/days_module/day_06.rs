use crate::days_module::day::Day;
use std::collections::{HashMap, HashSet};
pub struct Day06 {}

impl Day for Day06 {
    fn get_id(&self) -> String {
        "day_06".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        "Not implemented".to_string()
    }

    fn part_b(&self, input: &String) -> String {
        "Not implemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day06 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day06 {}.test_day_part(&'b')
    }
}
