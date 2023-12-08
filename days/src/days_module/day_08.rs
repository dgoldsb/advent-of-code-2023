use crate::days_module::day::Day;
use std::collections::HashMap;

pub struct Day08 {}

impl Day for Day08 {
    fn get_id(&self) -> String {
        "day_08".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let mut input_split = input.split("\n\n");

        let instructions = input_split.next().unwrap().chars().collect::<Vec<char>>();
        let raw_neigbors = input_split.next().unwrap();

        let mut neighbor_map = HashMap::new();
        for line in raw_neigbors.split("\n") {
            let nicer_line = line.replace(" = (", ", ").replace(")", "");
            let mut nicer_line_split = nicer_line.split(", ");

            let label = nicer_line_split.next().unwrap().to_string();
            let left = nicer_line_split.next().unwrap().to_string();
            let right = nicer_line_split.next().unwrap().to_string();

            neighbor_map.insert(label, (left, right));
        }

        let mut steps = 0;
        let mut current_node = "AAA";
        loop {
            let instruction = instructions[steps % instructions.len()];
            let neighbors = neighbor_map.get(current_node).unwrap();
            current_node = match instruction {
                'L' => &neighbors.0,
                'R' => &neighbors.1,
                _ => panic!(),
            };
            steps += 1;

            if current_node == "ZZZ" {
                break;
            }
        }

        steps.to_string()
    }

    fn part_b(&self, input: &String) -> String {
        return "Not implemented".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day08 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day08 {}.test_day_part(&'b')
    }
}
