use crate::days_module::day::Day;

pub struct Day12 {}

impl Day for Day12 {
    fn get_id(&self) -> String {
        "day_12".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        "0".to_string()
    }

    fn part_b(&self, input: &String) -> String {
        "0".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day12 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day12 {}.test_day_part(&'b')
    }
}
