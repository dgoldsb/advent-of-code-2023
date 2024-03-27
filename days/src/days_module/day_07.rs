use crate::days_module::day::Day;
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;

pub struct Day07 {}

lazy_static! {
    static ref CARD_VALUES: HashMap<char, i32> = {
        let mut map = HashMap::new();
        map.insert('2', 2);
        map.insert('3', 3);
        map.insert('4', 4);
        map.insert('5', 5);
        map.insert('6', 6);
        map.insert('7', 7);
        map.insert('8', 8);
        map.insert('9', 9);
        map.insert('T', 10); // Ten
        map.insert('Q', 12); // Queen
        map.insert('K', 13); // King
        map.insert('A', 14); // Ace
        map
    };
}

// Calculate a hand score.
fn score_hand(consider_jokers: &bool, cards: &Vec<char>) -> usize {
    let mut card_map: HashMap<char, usize> = HashMap::new();
    let mut joker_count = 0;

    // Count each card type in a hashmap.
    for card in cards {
        if *consider_jokers && card == &'J' {
            // Add one to the biggest number.
            joker_count += 1;
        } else {
            let card_count = card_map.get(card).unwrap_or(&0) + 1;
            card_map.insert(*card, card_count);
        }
    }

    // Convert the counts to a string an sort, this is our signature of the hand.
    let mut values_vec = card_map
        .iter()
        .map(|(_, v)| format!("{}", v))
        .collect::<Vec<String>>();
    values_vec.sort();

    // Add `n` jokers to the last, if there are no jokers this does nothing.
    let last_index = values_vec.len();
    if last_index == 0 {
        values_vec = ["5".to_string()].to_vec()
    } else {
        values_vec[last_index - 1] = format!(
            "{}",
            (values_vec[last_index - 1].parse::<usize>().unwrap() + joker_count)
        );
    }

    // Match the signature to a score.
    let signature_string = values_vec.iter().fold("".to_string(), |s, a| s + a);
    match signature_string.as_str() {
        "5" => 7,
        "14" => 6,
        "23" => 5,
        "113" => 4,
        "122" => 3,
        "1112" => 2,
        "11111" => 1,
        _ => panic!("Weird hand..."),
    }
}

#[derive(Eq, PartialEq, Ord)]
struct HandOfCards {
    cards: Vec<char>,
    bid_amount: usize,
    score: usize,
    consider_jokers: bool,
}

impl PartialOrd for HandOfCards {
    fn partial_cmp(&self, other: &HandOfCards) -> Option<Ordering> {
        match self.score.cmp(&other.score) {
            Ordering::Equal => {}
            Ordering::Greater => return Some(Ordering::Greater),
            Ordering::Less => return Some(Ordering::Less),
        }

        let default = if self.consider_jokers { 1 } else { 11 };

        for (own, other) in zip(&self.cards, &other.cards) {
            match CARD_VALUES
                .get(own)
                .unwrap_or(&default)
                .cmp(&CARD_VALUES.get(other).unwrap_or(&default))
            {
                Ordering::Equal => {}
                Ordering::Greater => return Some(Ordering::Greater),
                Ordering::Less => return Some(Ordering::Less),
            }
        }

        Some(Ordering::Equal)
    }
}

fn deserialize_hand(raw: &str, consider_jokers: &bool) -> HandOfCards {
    let mut input_split = raw.split(" ");
    let cards = input_split.next().unwrap().chars().collect::<Vec<char>>();
    return HandOfCards {
        cards: cards.clone(),
        bid_amount: input_split.next().unwrap().parse().unwrap(),
        score: score_hand(consider_jokers, &cards),
        consider_jokers: *consider_jokers,
    };
}

fn solve(input: &String, consider_jokers: &bool) -> String {
    let mut hands = input
        .split("\n")
        .map(|s| deserialize_hand(s, consider_jokers))
        .collect::<Vec<HandOfCards>>();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid_amount)
        .sum::<usize>()
        .to_string()
}

impl Day for Day07 {
    fn get_id(&self) -> String {
        "day_07".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        solve(input, &false)
    }

    fn part_b(&self, input: &String) -> String {
        solve(input, &true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day07 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day07 {}.test_day_part(&'b')
    }
}
