use crate::days_module::day::Day;
use helpers::transpose_string;
use std::cmp::min;

// TODO: Could be an iter to preserve CPU and memory.
fn get_variations(image: &str) -> Vec<String> {
    let mut result = Vec::new();
    for (i, c) in image.chars().enumerate() {
        let new_image = image
            .chars()
            .enumerate()
            .map(|(i_, c_)| {
                if i == i_ {
                    match c {
                        '#' => '.',
                        '.' => '#',
                        _ => c_,
                    }
                } else {
                    c_
                }
            })
            .collect::<String>();
        if new_image != *image {
            result.push(new_image);
        }
    }
    result
}

fn score_image(image: &str, transposed: bool, pass: Option<i32>) -> Option<i32> {
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
            let mut result = split.try_into().unwrap();
            if !transposed {
                result *= 100;
            }

            if pass.is_some() && pass.unwrap() == result {
                continue;
            }

            return Some(result);
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
            let score = match score_image(image, false, None) {
                Some(score) => score,
                None => match score_image(&transpose_string(image), true, None) {
                    Some(score) => score,
                    None => panic!("No score found!"),
                },
            };
            scores.push(score);
        }
        return scores.iter().sum::<i32>().to_string();
    }

    fn part_b(&self, input: &String) -> String {
        let mut scores = Vec::new();
        for image in input.split("\n\n") {
            let original_score = match score_image(image, false, None) {
                Some(score) => score,
                None => match score_image(&transpose_string(image), true, None) {
                    Some(score) => score,
                    None => panic!("No score found!"),
                },
            };

            let mut variations = get_variations(image);
            variations.reverse();
            let mut found = false;
            for variation in variations {
                let new_score_option = match score_image(&variation, false, Some(original_score)) {
                    Some(score) => Some(score),
                    None => {
                        match score_image(&transpose_string(&variation), true, Some(original_score))
                        {
                            Some(score) => Some(score),
                            None => None,
                        }
                    }
                };

                if new_score_option.is_none() {
                    continue;
                }

                if new_score_option.unwrap() != original_score {
                    scores.push(new_score_option.unwrap());
                    found = true;
                    break;
                }
            }
            if !found {
                panic!("No smudge found")
            }
        }
        return scores.iter().sum::<i32>().to_string();
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
