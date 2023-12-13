use crate::days_module::day::Day;
use helpers::transpose_string;
use std::cmp::min;

fn score_image(image: &str) -> Option<i32> {
    let lines = image.split("\n").collect::<Vec<&str>>();

    let len = lines.len();
    for split in 1..(lines.len() - 1) {
        let first_slice = &lines[..split];
        let second_slice = &lines[split..];

        let mut is_match = true;
        for i in 0..min(first_slice.len(), second_slice.len()) {
            let left_line = first_slice[first_slice.len() - i - 1];
            let right_line = second_slice[i];

            if left_line != right_line {
                is_match = false;
            }
        }

        if is_match {
            return Some(split.try_into().unwrap());
        };
    }
    None
}

pub struct Day13 {}

impl Day for Day13 {
    fn get_id(&self) -> String {
        "day_13".to_string()
    }

    // 34974 too high
    // 20527 too low
    fn part_a(&self, input: &String) -> String {
        let regular_sum = input
            .split("\n\n")
            .map(|image| score_image(image))
            .filter(|option| option.is_some())
            .map(|option| option.unwrap())
            .sum::<i32>();
        let transposed_sum = input
            .split("\n\n")
            .map(|image| score_image(&transpose_string(image)))
            .filter(|option| option.is_some())
            .map(|option| option.unwrap())
            .sum::<i32>();
        return (transposed_sum + 100 * regular_sum).to_string();
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
        Day13 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day13 {}.test_day_part(&'b')
    }
}
