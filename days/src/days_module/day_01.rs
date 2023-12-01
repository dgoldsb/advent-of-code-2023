use crate::days_module::day::Day;

pub struct Day01 {}

fn solve(input: &String) -> String {
    let mut first_number_cache: Option<char> = Option::None;
    let mut last_number_cache: Option<char> = Option::None;
    let mut ints: Vec<i32> = Vec::new();

    for input_char in input.chars() {
        match input_char {
            '\n' => {
                ints.push(
                    format!(
                        "{}{}",
                        first_number_cache.unwrap_or_default(),
                        last_number_cache.unwrap_or_default(),
                    )
                    .parse()
                    .expect("Should be an integer"),
                );
                first_number_cache = Option::None;
            }
            '0'..='9' => {
                if first_number_cache.is_none() {
                    first_number_cache = Option::Some(input_char)
                }
                last_number_cache = Option::Some(input_char)
            }
            _ => {}
        }
    }

    // Handle the last line, that may not have a newline.
    if first_number_cache.is_some() {
        ints.push(
            format!(
                "{}{}",
                first_number_cache.unwrap_or_default(),
                last_number_cache.unwrap_or_default(),
            )
            .parse()
            .expect("Should be an integer"),
        );
    }

    ints.iter().sum::<i32>().to_string()
}

impl Day for Day01 {
    fn get_id(&self) -> String {
        "day_01".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        solve(input)
    }

    fn part_b(&self, input: &String) -> String {
        // Parse in the integer, leaving the word-version intact on both sides to deal with
        // overlapping words.
        let owned_input = input
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");
        solve(&owned_input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day01 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day01 {}.test_day_part(&'b')
    }
}
