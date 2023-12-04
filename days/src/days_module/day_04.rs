use crate::days_module::day::Day;
use std::collections::{HashMap, HashSet};
pub struct Day04 {}

fn find_overlap(card: &str) -> i32 {
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

    matches_length.try_into().unwrap()
}

fn expand_cards(cards: &String) -> i32 {
    let mut total_cards = 0;
    let mut rewards = HashMap::new();

    for (index, card) in cards.split("\n").enumerate() {
        let current_card_count = 1 + rewards.get(&index).unwrap_or(&0);
        total_cards += current_card_count;
        for index_delta in 1..=find_overlap(card) {
            let reward_index: usize =
                index + <i32 as TryInto<usize>>::try_into(index_delta).unwrap();
            let new_rewards = rewards.get(&reward_index).unwrap_or(&0) + current_card_count;
            rewards.insert(reward_index, new_rewards);
        }
    }

    total_cards
}

fn score_card(card: &str) -> i32 {
    let matches_length = find_overlap(card);

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
        expand_cards(input).to_string()
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
