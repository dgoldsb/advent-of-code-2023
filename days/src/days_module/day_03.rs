use crate::days_module::day::Day;
use std::collections::{HashMap, HashSet};

pub struct Day03 {}

// TODO: To helpers.
fn moore_neighborhood(coordinate: &(i32, i32)) -> Vec<(i32, i32)> {
    let mut result = Vec::new();

    for i in -1..=1 {
        for j in -1..=1 {
            if i == j && i == 0 {
                continue;
            }
            result.push((coordinate.0 + i, coordinate.1 + j));
        }
    }

    result
}

// TODO: To helpers if useful again, as iter trait of input struct.
fn iterate(input: &String) -> Vec<((i32, i32), char)> {
    let mut result = Vec::new();

    let mut row_index = 0;
    for line in input.split("\n") {
        let mut column_index = 0;
        for char_ in line.chars() {
            result.push(((row_index, column_index), char_));
            column_index += 1;
        }
        row_index += 1;
    }

    result
}

fn is_symbol(char_: &char) -> bool {
    char_ != &'.' && !char_.is_digit(10)
}

fn extract_part_numbers(input: &String) -> Vec<i32> {
    let mut result = Vec::new();

    // First get indices of all symbols.
    let valid_squares = iterate(input)
        .iter()
        .filter(|x| is_symbol(&x.1))
        .map(|x| moore_neighborhood(&x.0))
        .flatten()
        .collect::<HashSet<(i32, i32)>>();

    // Second pass through the file, persisting only numbers adjacent to a symbol.
    let mut digit_buffer = Vec::new();
    let mut index_buffer = HashSet::new();
    for (index, char_) in iterate(input) {
        // If index is a new row, then flush buffer if any.
        // If not a digit, flush buffer.
        if (index.1 == 0 || !char_.is_digit(10)) && !index_buffer.is_empty() {
            if !index_buffer.is_disjoint(&valid_squares) {
                result.push(
                    digit_buffer
                        .iter()
                        .cloned()
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap(),
                )
            }

            index_buffer.clear();
            digit_buffer.clear();
        }

        // If it is a digit, add to buffer.
        if char_.is_digit(10) {
            digit_buffer.push(char_);
            index_buffer.insert(index);
        }
    }

    result
}

// TODO: Deduplicate.
fn extract_gear_ratios(input: &String) -> Vec<i32> {
    // First get indices of all gears.
    let gears = iterate(input)
        .iter()
        .filter(|x| x.1 == '*')
        .map(|x| x.0)
        .collect::<HashSet<(i32, i32)>>();

    let mut gear_ratios = HashMap::new();
    let mut multiplication_counts = HashMap::new();

    for gear in &gears {
        gear_ratios.insert(gear.clone(), 1);
        multiplication_counts.insert(gear.clone(), 0);
    }

    // Second pass through the file, persisting only numbers adjacent to a symbol.
    let mut digit_buffer = Vec::new();
    let mut index_buffer = HashSet::new();
    for (index, char_) in iterate(input) {
        // If index is a new row, then flush buffer if any.
        // If not a digit, flush buffer.
        if (index.1 == 0 || !char_.is_digit(10)) && !index_buffer.is_empty() {
            let moore_buffer = index_buffer
                .iter()
                .map(moore_neighborhood)
                .flatten()
                .collect::<HashSet<(i32, i32)>>();

            if !moore_buffer.is_disjoint(&gears) {
                for gear in &gears {
                    if moore_buffer.contains(gear) {
                        let number = digit_buffer
                            .iter()
                            .cloned()
                            .collect::<String>()
                            .parse::<i32>()
                            .unwrap();
                        let new_ratio = gear_ratios.get(&gear).unwrap_or(&1) * number;
                        gear_ratios.insert(*gear, new_ratio);
                        let new_multiplication_count =
                            multiplication_counts.get(&gear).unwrap_or(&0) + 1;
                        multiplication_counts.insert(*gear, new_multiplication_count);
                    }
                }
            }

            index_buffer.clear();
            digit_buffer.clear();
        }

        // If it is a digit, add to buffer.
        if char_.is_digit(10) {
            digit_buffer.push(char_);
            index_buffer.insert(index);
        }
    }

    gear_ratios
        .iter()
        .filter(|x| multiplication_counts.get(x.0).unwrap() == &2)
        .map(|x| *x.1)
        .collect()
}

impl Day for Day03 {
    fn get_id(&self) -> String {
        "day_03".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        extract_part_numbers(input).iter().sum::<i32>().to_string()
    }

    fn part_b(&self, input: &String) -> String {
        extract_gear_ratios(input).iter().sum::<i32>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day03 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day03 {}.test_day_part(&'b')
    }
}
