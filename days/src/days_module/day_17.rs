use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

// We are doing a weird A*, our base node consists of coordinates, and some context how we got here.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Node {
    coordinate: (i32, i32),
    delta: (i32, i32),
    streak: usize,
}

impl Node {
    fn get_neighbors(&self) -> Vec<Node> {
        let mut neighbors = Vec::new();
        for delta in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
            if delta.0 == -1 * self.delta.0 && delta.1 == -1 * self.delta.1 {
                continue;
            }

            let new_streak = if delta == self.delta {
                self.streak + 1
            } else {
                0
            };
            if new_streak >= 3 {
                continue;
            }

            neighbors.push(Node {
                coordinate: (self.coordinate.0 + delta.0, self.coordinate.1 + delta.1),
                delta: delta,
                streak: new_streak,
            });
        }
        neighbors
    }
}

// Returns the cost to reach the end.
fn a_star(start: (i32, i32), end: (i32, i32), enter_cost_grid: &Grid) -> u32 {
    let mut g_score: HashMap<Node, u32> = HashMap::new();
    let mut came_from: HashMap<Node, Node> = HashMap::new();
    let mut open_set: HashSet<Node> = HashSet::new();

    let mut queue = vec![
        Node {
            coordinate: start,
            delta: (0, 1),
            streak: 0,
        },
        Node {
            coordinate: start,
            delta: (1, 0),
            streak: 0,
        },
    ];

    let first_enter_cost = enter_cost_grid
        .get_cell_by_index(&start)
        .unwrap()
        .value
        .to_digit(10)
        .unwrap();

    g_score.insert(
        Node {
            coordinate: start,
            delta: (0, 1),
            streak: 0,
        },
        first_enter_cost,
    );
    g_score.insert(
        Node {
            coordinate: start,
            delta: (1, 0),
            streak: 0,
        },
        first_enter_cost,
    );

    for node in &queue {
        open_set.insert(*node);
    }

    while !queue.is_empty() {
        // Pop a node.
        let current_node = queue.pop().unwrap();
        open_set.remove(&current_node);

        // If we are at the exit it may be the lowest, or not. We do know we can stop this branch.
        if current_node.coordinate == end {
            continue;
        }

        for neighbor in current_node.get_neighbors() {
            // Calculate the enter cost.
            let enter_cost_option = enter_cost_grid.get_cell_by_index(&neighbor.coordinate);
            if enter_cost_option.is_none() {
                continue;
            }
            let enter_cost = enter_cost_option.unwrap().value.to_digit(10).unwrap();

            // Find how long we took to get to the predecessor.
            let predecessor = g_score.get(&current_node).unwrap();

            // Calculate the tentative G score.
            let tentative_g_score = *predecessor + enter_cost;

            // Continue if we have found a fast route here.
            if tentative_g_score > *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                continue;
            }

            // Update maps.
            came_from.insert(neighbor, current_node);
            g_score.insert(neighbor, tentative_g_score);

            if !open_set.contains(&neighbor) {
                open_set.insert(neighbor);
                queue.push(neighbor);
            }
        }
    }

    // Find the lowest terminal location.
    g_score
        .iter()
        .filter(|(k, _)| k.coordinate == end)
        .map(|(_, v)| v.clone())
        .min()
        .unwrap()
}

pub struct Day17 {}

impl Day for Day17 {
    fn get_id(&self) -> String {
        "day_17".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let grid = Grid::from_str(input).unwrap();
        let size: i32 = (input.split("\n").next().unwrap().len() - 1)
            .try_into()
            .unwrap();
        a_star((0, 0), (size, size), &grid).to_string()
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
        Day17 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day17 {}.test_day_part(&'b')
    }
}
