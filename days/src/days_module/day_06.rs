use crate::days_module::day::Day;
use helpers::ints_from_string;
use std::collections::{HashMap, HashSet};
use std::iter::zip;

// Solve ABC-formula.
fn find_intersections(time: &isize, distance: &isize) -> (f64, f64) {
    let positive =
        (-(*time as f64) + (((time).pow(2) - (4 * -1 * -distance)) as f64).sqrt()) / -2.0;
    let negative =
        (-(*time as f64) - (((time).pow(2) - (4 * -1 * -distance)) as f64).sqrt()) / -2.0;
    return (positive + 0.000001, negative - 0.000001); // hack to handle exact integer intersections
}

// Count the discrete number of ms input to win the race.
fn count_number_of_ways(time: &isize, distance: &isize) -> usize {
    let float_tuple = find_intersections(time, distance);
    let int_tuple = (
        float_tuple.0.ceil() as isize,
        float_tuple.1.floor() as isize,
    );
    let result = (int_tuple.1 - int_tuple.0 + 1).try_into().unwrap();
    result
}

fn parse_input(input: &String) -> Vec<(isize, isize)> {
    let mut input_iterator = input.split("\n");

    let times = ints_from_string(input_iterator.next().expect("Expected times"));
    let distances = ints_from_string(input_iterator.next().expect("Expected distances"));

    zip(times, distances).collect::<Vec<(isize, isize)>>()
}

fn solve(input: &String) -> String {
    parse_input(input)
        .iter()
        .map(|t| count_number_of_ways(&t.0, &t.1))
        .product::<usize>()
        .to_string()
}

pub struct Day06 {}

impl Day for Day06 {
    fn get_id(&self) -> String {
        "day_06".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        solve(input)
    }

    fn part_b(&self, input: &String) -> String {
        solve(&input.replace(" ", "").replace(":", ": "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day06 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day06 {}.test_day_part(&'b')
    }
}
