use crate::days_module::day::Day;


pub struct Day11 {}

impl Day for Day11 {
    fn get_id(&self) -> String {
        "day_11".to_string()
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
        Day11 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day11 {}.test_day_part(&'b')
    }
}
