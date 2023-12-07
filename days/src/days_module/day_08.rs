use crate::days_module::day::Day;

pub struct Day08 {}

impl Day for Day08 {
    fn get_id(&self) -> String {
        "day_08".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        return "Not implemented".to_string()
    }

    fn part_b(&self, input: &String) -> String {
        return "Not implemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day08 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day08 {}.test_day_part(&'b')
    }
}
