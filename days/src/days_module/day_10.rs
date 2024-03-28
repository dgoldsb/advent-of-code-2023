use crate::days_module::day::Day;
use helpers::grid::cell::Cell;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::GridIndex;
use std::collections::HashSet;
use std::str::FromStr;

fn find_starting_char(index: &(isize, isize), map: &Grid) -> char {
    let default = Cell {
        value: '.',
        index: GridIndex { x: -1, y: -1 },
    };
    let up = &map
        .get_cell_by_index(&(index.0 - 1, index.1))
        .unwrap_or(&default)
        .value;
    let down = &map
        .get_cell_by_index(&(index.0 + 1, index.1))
        .unwrap_or(&default)
        .value;
    let left = &map
        .get_cell_by_index(&(index.0, index.1 - 1))
        .unwrap_or(&default)
        .value;
    let right = &map
        .get_cell_by_index(&(index.0, index.1 + 1))
        .unwrap_or(&default)
        .value;

    if vec!['L', 'F', '-'].contains(left) {
        if vec!['F', '7', '|'].contains(up) {
            return 'J';
        } else if vec!['7', 'J', '-'].contains(right) {
            return '-';
        } else {
            return '7';
        }
    }

    if vec!['F', '7', '|'].contains(up) {
        if vec!['L', 'J', '|'].contains(down) {
            return '|';
        } else {
            return 'L';
        }
    }

    return 'F';
}

fn get_neighbors(pipe: &char, index: &(isize, isize)) -> Vec<(isize, isize)> {
    match pipe {
        '|' => {
            vec![(index.0 + 1, index.1), (index.0 - 1, index.1)]
        }
        '-' => {
            vec![(index.0, index.1 + 1), (index.0, index.1 - 1)]
        }
        'L' => {
            vec![(index.0 - 1, index.1), (index.0, index.1 + 1)]
        }
        'J' => {
            vec![(index.0 - 1, index.1), (index.0, index.1 - 1)]
        }
        '7' => {
            vec![(index.0 + 1, index.1), (index.0, index.1 - 1)]
        }
        'F' => {
            vec![(index.0 + 1, index.1), (index.0, index.1 + 1)]
        }
        _ => panic!("Invalid pipe piece!"),
    }
}

fn find_loop(map: &Grid) -> HashSet<(isize, isize)> {
    let mut visited = HashSet::new();
    let grid_index = map.find_index(&'S').unwrap().clone();
    let mut index = (grid_index.x, grid_index.y);
    let mut char_ = find_starting_char(&index, map);

    loop {
        visited.insert(index.clone());

        let mut updated = false;
        for neighbor in get_neighbors(&char_, &index) {
            if !visited.contains(&neighbor) {
                index = neighbor;
                char_ = map.get_cell_by_index(&index).unwrap().value;
                updated = true;
            }
        }

        if updated == false {
            break;
        }
    }
    visited
}

pub struct Day10 {}

impl Day for Day10 {
    fn get_id(&self) -> String {
        "day_10".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let map = Grid::from_str(input).unwrap();
        (find_loop(&map).len() / 2).to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let map = Grid::from_str(input).unwrap();
        let visited = find_loop(&map);
        let mut inner_squares = 0;
        for (x, line) in input.split("\n").enumerate() {
            let mut inside = false;
            let mut last_character: char = '.';

            for (y, input_char) in line.chars().enumerate() {
                if visited.contains(&(x as isize, y as isize)) {
                    // We need to track if we are inside.
                    inside = match input_char {
                        '|' => !inside,
                        'L' => {
                            last_character = 'L';
                            inside
                        }
                        '7' => {
                            if last_character == 'L' {
                                !inside
                            } else {
                                inside
                            }
                        }
                        'F' => {
                            last_character = 'F';
                            inside
                        }
                        'J' => {
                            if last_character == 'F' {
                                !inside
                            } else {
                                inside
                            }
                        }
                        _ => inside,
                    }
                } else {
                    if inside {
                        inner_squares += 1;
                    }
                }
            }
        }
        inner_squares.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day10 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day10 {}.test_day_part(&'b')
    }
}
