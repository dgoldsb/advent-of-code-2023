use crate::days_module::day::Day;
use helpers::transpose_string;
use std::cmp::min;

fn slice_to_bits(input_slice: &str) -> usize {
    // Convert slice to binary representation.
    let input_map = input_slice
        .chars()
        .filter(|c| *c == '#' || *c == '.')
        .map(|c| match c {
            '#' => '1',
            '.' => '0',
            _ => panic!("Invalid character in input slice"),
        });

    // If we want to revert the input, this is the time.
    let binary_representation: String = input_map.collect();

    // Convert binary representation to integer.
    let binary_integer = usize::from_str_radix(&binary_representation, 2).unwrap();

    binary_integer
}

// Return the number of differences between two lines.
fn compare_image_lines(first: &str, second: &str) -> u32 {
    let first_slice_binary = slice_to_bits(first);
    let second_slice_binary = slice_to_bits(second);

    let xor_result = first_slice_binary ^ second_slice_binary;
    let ones_count = xor_result.count_ones();

    return ones_count;
}

// Return the number of differences when mirroring on `split_index`.
fn test_mirror_split(lines: &Vec<&str>, split_index: usize) -> u32 {
    let mut result = 0;

    let first_slice = &lines[..split_index];
    let second_slice = &lines[split_index..];

    for i in 0..min(first_slice.len(), second_slice.len()) {
        let left_line = first_slice[first_slice.len() - i - 1];
        let right_line = second_slice[i];

        result += compare_image_lines(left_line, right_line);
    }

    result
}

// Finds the index of the split where te image is a perfect mirror image, with
// `target_difference_count` discrepancies.
fn find_mirror_split(image: &str, target_difference_count: u32) -> Option<usize> {
    let lines = image.split("\n").collect::<Vec<&str>>();
    (1..lines.len())
        .map(|n| (n, test_mirror_split(&lines, n)))
        .filter(|(_, v)| *v == target_difference_count)
        .map(|(n, _)| n)
        .next()
}

// Score an image, the score is the index of its "mirror line". If it needs to be transposed the
// score is multiplied by `100`.
fn score_image(image: &str, target_difference_count: u32) -> usize {
    match find_mirror_split(image, target_difference_count) {
        Some(s) => s * 100,
        None => score_image(&transpose_string(image), target_difference_count) / 100,
    }
}

pub struct Day13 {}

impl Day for Day13 {
    fn get_id(&self) -> String {
        "day_13".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let mut scores = Vec::new();
        for image in input.split("\n\n") {
            scores.push(score_image(image, 0));
        }
        return scores.iter().sum::<usize>().to_string();
    }

    fn part_b(&self, input: &String) -> String {
        let mut scores = Vec::new();
        for image in input.split("\n\n") {
            scores.push(score_image(image, 1));
        }
        return scores.iter().sum::<usize>().to_string();
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
