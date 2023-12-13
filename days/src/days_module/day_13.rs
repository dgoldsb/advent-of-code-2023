use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::GridIndex;
use std::collections::HashSet;
use std::str::FromStr;

fn score_image(image: &str) -> (i32, i32) {
    let points = Grid::from_str(image)
        .unwrap()
        .iter()
        .filter(|cell| cell.value == '#')
        .map(|cell| cell.index)
        .collect::<HashSet<GridIndex>>();

    // Check for vertical reflection.
    for x_mirror in 1..i32::MAX {
        let left_of_mirror = points
            .iter()
            .filter(|point| point.x < x_mirror)
            .map(|point| GridIndex { x: point.x, y: point.y })
            .collect::<HashSet<GridIndex>>();


        let right_of_mirror = points
            .iter()
            .filter(|point| point.x > x_mirror)
            .map(|point| GridIndex { x: point.x - 2 * (point.x - x_mirror) - 1, y: point.y })
            .collect::<HashSet<GridIndex>>();

        if right_of_mirror.is_empty() {
            break
        }

        if right_of_mirror.is_subset(&left_of_mirror) || left_of_mirror.is_subset(&right_of_mirror) {
            return (x_mirror, 0);
        }
    }

    // Check for horizontal reflection.
    for y_mirror in 1..i32::MAX {
        let left_of_mirror = points
            .iter()
            .filter(|point| point.y < y_mirror)
            .map(|point| GridIndex { x: point.x, y: point.y })
            .collect::<HashSet<GridIndex>>();


        let right_of_mirror = points
            .iter()
            .filter(|point| point.y > y_mirror)
            .map(|point| GridIndex { x: point.x, y: point.y - 2 * (point.y - y_mirror) - 1 })
            .collect::<HashSet<GridIndex>>();

        if right_of_mirror.is_empty() {
            break
        }

        if right_of_mirror.is_subset(&left_of_mirror) || left_of_mirror.is_subset(&right_of_mirror) {
            return (0, y_mirror);
        }
    }

    panic!("No reflection found!")
}

pub struct Day13 {}

impl Day for Day13 {
    fn get_id(&self) -> String {
        "day_13".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let summary = input
            .split("\n\n")
            .map(|image| score_image(image))
            .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
        return (summary.1 + 100 * summary.0).to_string();
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
        Day13 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day13 {}.test_day_part(&'b')
    }
}
