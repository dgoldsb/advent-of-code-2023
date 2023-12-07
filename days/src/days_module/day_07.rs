use crate::days_module::day::Day;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;
use std::str::FromStr;

pub struct Day07 {}

#[derive(Eq, PartialEq, Ord)]
struct HandOfCards {
    cards: Vec<char>,
    bid_amount: usize,
}

impl HandOfCards {
    fn find_hand_type(&self) -> usize {
        let mut card_map: HashMap<char, usize> = HashMap::new();

        for card in &self.cards {
            let card_count = card_map.get(card).unwrap_or(&0) + 1;
            card_map.insert(*card, card_count);
        }

        let mut values_vec = card_map
            .iter()
            .map(|(_, v)| format!("{}", v))
            .collect::<Vec<String>>();
        values_vec.sort();
        let signature = values_vec.iter().fold("".to_string(), |s, a| s + a);

        match signature.as_str() {
            "5" => 7,
            "14" => 6,
            "23" => 5,
            "113" => 4,
            "122" => 3,
            "1112" => 2,
            "11111" => 2,
            _ => panic!("Weird hand..."),
        }
    }
}

impl FromStr for HandOfCards {
    type Err = ();

    fn from_str(input: &str) -> Result<HandOfCards, Self::Err> {
        let mut input_split = input.split(" ");
        return Ok(HandOfCards {
            cards: input_split.next().unwrap().chars().collect::<Vec<char>>(),
            bid_amount: input_split.next().unwrap().parse().unwrap(),
        });
    }
}

impl PartialOrd for HandOfCards {
    fn partial_cmp(&self, other: &HandOfCards) -> Option<Ordering> {
        fn card_value(card: &char) -> usize {
            match *card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => card.to_digit(10).unwrap().try_into().unwrap(),
            }
        }

        match self.find_hand_type().cmp(&other.find_hand_type()) {
            Ordering::Equal => {},
            Ordering::Greater => return Some(Ordering::Greater),
            Ordering::Less => return Some(Ordering::Less),
        }

        for (own, other) in zip(&self.cards, &other.cards) {
            match card_value(own).cmp(&card_value(other)) {
                Ordering::Equal => {},
                Ordering::Greater => return Some(Ordering::Greater),
                Ordering::Less => return Some(Ordering::Less),
            }
        }

        panic!("No valid ordering...");
    }
}

impl Day for Day07 {
    fn get_id(&self) -> String {
        "day_07".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let mut hands = input
            .split("\n")
            .map(HandOfCards::from_str)
            .map(|r| r.unwrap())
            .collect::<Vec<HandOfCards>>();
        hands.sort();
        hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i + 1) * h.bid_amount)
            .sum::<usize>()
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
        Day07 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day07 {}.test_day_part(&'b')
    }
}
