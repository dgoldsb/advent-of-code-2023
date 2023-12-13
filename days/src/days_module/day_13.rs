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

    fn mirror(number: i32, mirror: i32) -> i32 {
        number - mirror
    }

    // Check for vertical reflection.
    for y_mirror in 1..i32::MAX {
        let shifted = points
            .iter()
            .map(|point| GridIndex {
                x: mirror(point.x, y_mirror),
                y: point.y,
            })
            .collect::<HashSet<GridIndex>>();

        let left_of_mirror = shifted
            .iter()
            .filter(|point| point.x < 0)
            .map(|point| GridIndex {
                x: point.x.abs(),
                y: point.y,
            })
            .collect::<HashSet<GridIndex>>();

        let right_of_mirror = shifted
            .iter()
            .filter(|point| point.x >= 0)
            .map(|point| GridIndex {
                x: point.x + 1,
                y: point.y,
            })
            .collect::<HashSet<GridIndex>>();

        if right_of_mirror.is_empty() {
            break;
        }

        if left_of_mirror.is_empty() {
            continue;
        }

        if right_of_mirror.is_subset(&left_of_mirror) || left_of_mirror.is_subset(&right_of_mirror)
        {
            return (y_mirror, 0);
        }
    }

    // Check for horizontal reflection.
    for x_mirror in 1..i32::MAX {
        let shifted = points
            .iter()
            .map(|point| GridIndex {
                x: point.x,
                y: mirror(point.y, x_mirror),
            })
            .collect::<HashSet<GridIndex>>();

        let left_of_mirror = shifted
            .iter()
            .filter(|point| point.y < 0)
            .map(|point| GridIndex {
                x: point.x,
                y: point.y.abs(),
            })
            .collect::<HashSet<GridIndex>>();

        let right_of_mirror = shifted
            .iter()
            .filter(|point| point.y >= 0)
            .map(|point| GridIndex {
                x: point.x,
                y: point.y + 1,
            })
            .collect::<HashSet<GridIndex>>();

        if right_of_mirror.is_empty() {
            break;
        }

        if left_of_mirror.is_empty() {
            continue;
        }

        if right_of_mirror.is_subset(&left_of_mirror) || left_of_mirror.is_subset(&right_of_mirror)
        {
            return (0, x_mirror);
        }
    }

    panic!("No reflection found!\n\n{}", image)
}

pub struct Day13 {}

impl Day for Day13 {
    fn get_id(&self) -> String {
        "day_13".to_string()
    }

    // 34974 too high
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
