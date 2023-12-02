use crate::days_module::day::Day;
use core::num::ParseIntError;
use std::collections::HashMap;

pub struct Day02 {}

fn extract_game_id(game_string: &str) -> Result<u32, ParseIntError> {
    // Example: `Game 3: 10 blue, ...`.
    let mut head_string_split = game_string.split(": ").next().unwrap().split(" ");
    head_string_split.next();
    head_string_split.next().unwrap().parse()
}

fn extract_game(game_string: &str) -> Vec<HashMap<String, u32>> {
    let mut rounds = Vec::new();

    let mut game_string_split = game_string.split(": ");
    game_string_split.next();

    for round_string in game_string_split.next().unwrap().split("; ") {
        let mut round = HashMap::new();

        for cube_string in round_string.split(", ") {
            let mut cube_string_split = cube_string.split(" ");
            let amount_string = cube_string_split.next().unwrap();
            let color = cube_string_split.next().unwrap();

            let amount = round.get(color).unwrap_or(&0) + amount_string.parse::<u32>().unwrap();
            round.insert(color.to_string(), amount);
        }
        rounds.push(round);
    }
    rounds
}

fn is_possible_round(game: &HashMap<String, u32>) -> bool {
    game.get(&"red".to_string()).unwrap_or(&0) <= &12
        && game.get(&"green".to_string()).unwrap_or(&0) <= &13
        && game.get(&"blue".to_string()).unwrap_or(&0) <= &14
}

impl Day for Day02 {
    fn get_id(&self) -> String {
        "day_02".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let mut sum_of_ids = 0;

        for line in input.split("\n") {
            if extract_game(line).iter().all(is_possible_round) {
                sum_of_ids += extract_game_id(line).unwrap();
            }
        }

        sum_of_ids.to_string()
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
        Day02 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day02 {}.test_day_part(&'b')
    }
}
