use crate::days_module::day::Day;
use helpers::transpose_string;
use std::cmp::min;

fn score_image(image: &str) -> Option<i32> {
    let lines = image.split("\n").collect::<Vec<&str>>();

    for split in 1..(lines.len()) {
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

    fn part_a(&self, input: &String) -> String {
        let mut scores = Vec::new();
        for image in input.split("\n\n") {
            let score = match score_image(image) {
                Some(score) => score * 100,
                None => match score_image(&transpose_string(image)) {
                    Some(score) => score,
                    None => panic!("No score found!"),
                },
            };
            scores.push(score);
        }
        return scores.iter().sum::<i32>().to_string();
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
