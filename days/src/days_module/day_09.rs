use crate::days_module::day::Day;
use helpers::ints_from_string;

pub struct Day09 {}

fn extrapolate_delta(series: &Vec<isize>) -> isize {
    // Base case.
    if series.iter().all(|i| i == &0) {
        return 0;
    }

    // Recurse.
    let differences = series
        .iter()
        .zip(series.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<isize>>();
    let delta = extrapolate_delta(&differences);
    let last_difference = differences.last().unwrap();
    return last_difference + delta
}

fn extrapolate(series: &Vec<isize>) -> isize {
    let last_number = *series.last().unwrap();
    let delta = extrapolate_delta(series);
    last_number + delta
}

impl Day for Day09 {
    fn get_id(&self) -> String {
        "day_09".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        input
            .split("\n")
            .map(ints_from_string)
            .map(|s| extrapolate(&s))
            .sum::<isize>()
            .to_string()
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
        Day09 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day09 {}.test_day_part(&'b')
    }
}
