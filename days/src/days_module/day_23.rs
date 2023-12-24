use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::GridIndex;
use std::cmp::max;
use std::collections::HashSet;
use std::str::FromStr;

// Again a custom state space: we cannot revisit places, which is an important pitvall because we
// are looking for a longest possible route.
#[derive(Clone, Eq, PartialEq, Hash)]
struct Node {
    coordinate: GridIndex,
    visited: Vec<GridIndex>,
}

impl Node {
    fn get_neighbors(&self, value_grid: &Grid, part_b: bool) -> Vec<Node> {
        let mut neighbors = Vec::new();

        // Get the value of the coordinate.
        let value = value_grid.get_cell(&self.coordinate).unwrap().value;

        for delta in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
            if !part_b && value == '>' && delta != (0, 1) {
                continue;
            }

            if !part_b && value == 'v' && delta != (1, 0) {
                continue;
            }

            let new_index = GridIndex {
                x: self.coordinate.x + delta.0,
                y: self.coordinate.y + delta.1,
            };

            // Get the value of the coordinate.
            let new_value_option = value_grid.get_cell(&new_index);
            let new_value;
            match new_value_option {
                Some(cell) => new_value = cell.value,
                None => continue,
            }

            if new_value == '#' {
                continue;
            }

            if self.visited.contains(&new_index) {
                continue;
            }

            let mut new_visited = self.visited.clone();
            new_visited.push(self.coordinate);

            neighbors.push(Node {
                coordinate: new_index,
                visited: new_visited,
            });
        }
        neighbors
    }
}

fn find_longest_path(start: GridIndex, end: GridIndex, value_grid: &Grid, part_b: bool) -> usize {
    let mut stack = vec![Node {
        coordinate: start,
        visited: vec![start],
    }];
    let mut longest_path = 0;

    while !stack.is_empty() {
        let state = stack.pop().unwrap();

        // Record and end if we found a path.
        if state.coordinate == end {
            longest_path = max(longest_path, state.visited.len());
            continue;
        }

        for neighbour in state.get_neighbors(value_grid, part_b) {
            stack.push(neighbour)
        }
    }
    longest_path - 1
}

pub struct Day23 {}

impl Day for Day23 {
    fn get_id(&self) -> String {
        "day_23".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let grid = Grid::from_str(input).unwrap();
        // TODO: Find the start and end programmatically instead of manual replacement.
        //  Now I manually replaced the start and end with S and E.
        let start = grid.find_index(&'S').expect("Add S at the start").clone();
        let end = grid.find_index(&'E').expect("Add E at the end").clone();

        find_longest_path(start, end, &grid, false).to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let grid = Grid::from_str(input).unwrap();
        // TODO: Find the start and end programmatically instead of manual replacement.
        //  Now I manually replaced the start and end with S and E.
        let start = grid.find_index(&'S').expect("Add S at the start").clone();
        let end = grid.find_index(&'E').expect("Add E at the end").clone();

        find_longest_path(start, end, &grid, true).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day23 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day23 {}.test_day_part(&'b')
    }
}
