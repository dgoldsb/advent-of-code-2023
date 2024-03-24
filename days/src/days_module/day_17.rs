use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

// We are doing a weird Dijkstra, our base node consists of coordinates, and some context how we got here.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
struct Node {
    coordinate: (isize, isize),
    delta: (isize, isize),
    streak: usize,
}

impl Node {
    fn get_neighbors(&self, ultra_crucible: bool) -> Vec<Node> {
        let min_streak = match ultra_crucible {
            true => 3,
            false => 0,
        };
        let max_streak = match ultra_crucible {
            true => 10,
            false => 3,
        };

        if self.streak < min_streak {
            return vec![Node {
                coordinate: (
                    self.coordinate.0 + self.delta.0,
                    self.coordinate.1 + self.delta.1,
                ),
                delta: self.delta,
                streak: self.streak + 1,
            }];
        }

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
            if new_streak >= max_streak {
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
fn find_path(
    start: (isize, isize),
    end: (isize, isize),
    enter_cost_grid: &Grid,
    ultra_crucible: bool,
) -> u32 {
    let mut g_score: HashMap<Node, u32> = HashMap::new();
    let mut came_from: HashMap<Node, Node> = HashMap::new();
    let mut open_set: HashSet<Node> = HashSet::new();
    let mut min_heap: BinaryHeap<(u32, Node)> = BinaryHeap::new();

    min_heap.push((
        u32::MAX,
        Node {
            coordinate: start,
            delta: (0, 1),
            streak: 0,
        },
    ));
    min_heap.push((
        u32::MAX,
        Node {
            coordinate: start,
            delta: (1, 0),
            streak: 0,
        },
    ));

    g_score.insert(
        Node {
            coordinate: start,
            delta: (0, 1),
            streak: 0,
        },
        0,
    );
    g_score.insert(
        Node {
            coordinate: start,
            delta: (1, 0),
            streak: 0,
        },
        0,
    );

    for (_, node) in &min_heap {
        open_set.insert(*node);
    }

    while !min_heap.is_empty() {
        // Pop a node.
        let (_, current_node) = min_heap.pop().unwrap();
        open_set.remove(&current_node);

        // If we are at the exit it may be the lowest, or not. We do know we can stop this branch.
        if current_node.coordinate == end {
            continue;
        }

        for neighbor in current_node.get_neighbors(ultra_crucible) {
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
                min_heap.push((u32::MAX - tentative_g_score, neighbor));
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
        let size: isize = (input.split("\n").next().unwrap().len() - 1)
            .try_into()
            .unwrap();
        find_path((0, 0), (size, size), &grid, false).to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let grid = Grid::from_str(input).unwrap();
        let size: isize = (input.split("\n").next().unwrap().len() - 1)
            .try_into()
            .unwrap();
        find_path((0, 0), (size, size), &grid, true).to_string()
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
