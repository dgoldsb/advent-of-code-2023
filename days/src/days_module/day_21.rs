use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::GridIndex;
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;

pub struct Day21 {}

impl Day for Day21 {
    fn get_id(&self) -> String {
        "day_21".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let grid = Grid::from_str(input).unwrap();
        let start = grid.find_index(&'S').unwrap().clone();

        let mut visited = HashSet::new();
        let mut end_plots = HashSet::new();

        // This needs to be a heap, because otherwise we prune branches that have more steps left
        // than the branch that inserted the index into `visited`.
        let mut heap: BinaryHeap<(i32, GridIndex)> = BinaryHeap::new();
        let mut in_heap = HashSet::new();
        heap.push((i32::MAX, start));
        in_heap.insert(start);

        let limit = 64;

        while !heap.is_empty() {
            let (non_inverted_steps, current) = heap.pop().unwrap();
            in_heap.remove(&current);
            let steps = i32::MAX - non_inverted_steps;

            // Add to visited.
            visited.insert(current);

            // Maybe add to end plots.
            if steps % 2 == 0 {
                end_plots.insert(current);
            }

            // Maybe add neighbours to the queue.
            let neighbours = current.neumann_neighborhood();
            for neighbour in neighbours {
                let cell_option = grid.get_cell(&neighbour);

                // Skip is it is not in the grid.
                if cell_option.is_none() {
                    continue;
                }

                // Skip if this is a `#`.
                if grid.get_cell(&neighbour).unwrap().value == '#' {
                    continue;
                }

                // Skip if visited.
                if visited.contains(&neighbour) {
                    continue;
                }

                // Skip if over steps limit.
                if steps == limit {
                    continue;
                }

                // Skip if it is in the heap.
                if in_heap.contains(&neighbour) {
                    continue;
                }

                heap.push((i32::MAX - (steps + 1), neighbour));
                in_heap.insert(neighbour);
            }
        }

        end_plots.len().to_string()
    }

    // I need to normalize to read the value.
    // In map we need to actually put the real unnormalized.
    // This is clearly too much to actually traverse... Or is it?
    // Maybe it is possible already with our visited map? No, our visited will be gigabytes.
    // Maybe it follows a curve?
    // Or maybe we need smart memoization...
    fn part_b(&self, input: &String) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day21 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day21 {}.test_day_part(&'b')
    }
}
