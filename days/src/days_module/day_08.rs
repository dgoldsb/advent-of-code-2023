use crate::days_module::day::Day;
use std::collections::HashMap;

struct Node<'a> {
    label: String,
    // Order matters.
    neighbors: Vec<&'a Node<'a>>,
}

impl<'a> Node<'a> {
    fn add_neighbor<'b>(&'b mut self, neighbor: &'b Node) where 'b: 'a {
        self.neighbors.push(neighbor);
    }
}

pub struct Day08 {}

impl Day for Day08 {
    fn get_id(&self) -> String {
        "day_08".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let mut input_split = input.split("\n\n");

        let instructions = input_split.next().unwrap();

        let mut node_map = HashMap::new();

        for line in input.split("\n") {
            let label = line.split(" ").next().unwrap().to_string();
            node_map.insert(label.clone(), Node { label, neighbors: Vec::new() });
        }


        for line in input.split("\n") {
            let nicer_line = line.replace(" = (", ", ").replace(")", "");
            let mut nicer_line_split = nicer_line.split(", ");

            let label = nicer_line_split.next().unwrap();
            let left = nicer_line_split.next().unwrap();
            let right = nicer_line_split.next().unwrap();

            let node:  &mut Node<'_> = node_map.get_mut(label).unwrap();
            node.add_neighbor(node_map.get(left).unwrap());
            node.add_neighbor(node_map.get(right).unwrap());
        }

        let start_node = node_map.get("AAA").expect("Expecting start node AAA");

        return "Not implemented".to_string()
    }

    fn part_b(&self, input: &String) -> String {
        return "Not implemented".to_string()
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
