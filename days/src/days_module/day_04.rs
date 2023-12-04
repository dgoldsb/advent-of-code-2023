use crate::days_module::day::Day;
use std::collections::HashSet;
pub struct Day04 {}

fn score_card(card: &str) -> i32 {
    let mut card_split = card.split(": ");
    card_split.next();
    let mut number_split = card_split.next().unwrap().split(" | ");

    // Find the winning numbers.
    let winning_numbers: HashSet<i32> = number_split
        .next()
        .unwrap()
        .split(" ")
        .map(|s| s.parse())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect::<HashSet<i32>>();

    // Find the card numbers.
    let card_numbers: HashSet<i32> = number_split
        .next()
        .unwrap()
        .split(" ")
        .map(|s| s.parse())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect::<HashSet<i32>>();

    // Find the overlap size.
    let matches_length = winning_numbers
        .intersection(&card_numbers)
        .collect::<Vec<&i32>>()
        .len();

    if matches_length > 0 {
        let base: i32 = 2;
        return base.pow((matches_length - 1).try_into().unwrap());
    }
    return 0;
}

impl Day for Day04 {
    fn get_id(&self) -> String {
        "day_04".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        input
            .split("\n")
            .map(|card| score_card(card))
            .sum::<i32>()
            .to_string()
    }

    fn part_b(&self, input: &String) -> String {
        "Not implemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day04 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day04 {}.test_day_part(&'b')
    }
}
