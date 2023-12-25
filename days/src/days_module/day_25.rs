use crate::days_module::day::Day;

pub struct Day20 {}

impl Day for Day20 {
    fn get_id(&self) -> String {
        "day_20".to_string()
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
        Day20 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day20 {}.test_day_part(&'b')
    }
}
