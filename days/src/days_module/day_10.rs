use crate::days_module::day::Day;
use std::collections::{HashMap, HashSet};

// TODO: Put on struct?
fn find_char_index<'a>(target: char, map: &'a HashMap<(isize, isize), char>) -> &'a (isize, isize) {
    for (key, val) in map.iter() {
        if val == &target {
            return key;
        }
    }
    panic!("Target not found!")
}

pub fn parse_char_map(input: &String) -> HashMap<(isize, isize), char> {
    let mut map = HashMap::new();
    for (i, l) in input.split("\n").enumerate() {
        for (j, v) in l.chars().enumerate() {
            map.insert((i as isize, j as isize), v.clone());
        }
    }
    map
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

pub struct Day10 {}

impl Day for Day10 {
    fn get_id(&self) -> String {
        "day_10".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let map = parse_char_map(input);

        let mut visited = HashSet::new();
        let mut index = find_char_index('S', &map).clone();
        // TODO: Replace with starting char.
        let mut char_ = '7';

        loop {
            visited.insert(index.clone());

            let mut updated = false;
            for neighbor in get_neighbors(&char_, &index) {
                if !visited.contains(&neighbor) {
                    index = neighbor;
                    char_ = *map.get(&index).unwrap();
                    updated = true;
                }
            }

            if updated == false {
                break;
            }
        }

        (visited.len() / 2).to_string()
    }

    fn part_b(&self, input: &String) -> String {
        // TODO: Part A, factor out.
        let map = parse_char_map(input);
        let mut visited = HashSet::new();
        let mut index = find_char_index('S', &map).clone();
        let mut char_ = '7';

        loop {
            visited.insert(index.clone());

            let mut updated = false;
            for neighbor in get_neighbors(&char_, &index) {
                if !visited.contains(&neighbor) {
                    index = neighbor;
                    char_ = *map.get(&index).unwrap();
                    updated = true;
                }
            }

            if updated == false {
                break;
            }
        }

        // TODO: Now for part B.
        let mut inner_squares = 0;
        for (x, line) in input.split("\n").enumerate() {
            let mut pipes = 0;
            let mut els = 0;
            let mut sevens = 0;
            let mut efs = 0;
            let mut jays = 0;

            let mut inside = false;
            let mut last_character: char = '.';

            for (y, input_char) in line.chars().enumerate() {
                if visited.contains(&(x as isize, y as isize)) {
                    // We need to track if we are inside.
                    // TODO: This needs to be cleaned up... A lot...
                    inside = match input_char {
                        '|' => !inside,
                        'L' => {
                            last_character = 'L';
                            inside
                        }
                        '7' => {
                            if last_character == 'L' {
                                !inside
                            } else if last_character == 'F' {
                                inside
                            } else {
                                panic!("What!")
                            }
                        }
                        'F' => {
                            last_character = 'F';
                            inside
                        }
                        'J' => {
                            if last_character == 'F' {
                                !inside
                            } else if last_character == 'L' {
                                inside
                            } else {
                                panic!("What!")
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
