use crate::days_module::day::Day;

fn hash(string: &str) -> usize {
    let mut current_value = 0;
    for c in string.chars() {
        let ascii_value = c as usize;
        current_value += ascii_value;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

pub struct Day15 {}

impl Day for Day15 {
    fn get_id(&self) -> String {
        "day_15".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        input.split(",").map(hash).sum::<usize>().to_string()
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
        Day15 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day15 {}.test_day_part(&'b')
    }
}
