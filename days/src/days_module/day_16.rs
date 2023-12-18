use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::GridIndex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub struct Day16 {}

fn solve(input: &String, start: GridIndex, start_delta: (i32, i32)) -> usize {
    let grid = Grid::from_str(input).unwrap();

    let mut beams: HashMap<GridIndex, HashSet<(i32, i32)>> = HashMap::new();
    let mut queue: Vec<(GridIndex, (i32, i32))> = Vec::new();

    queue.push((start, start_delta));

    while !queue.is_empty() {
        let (loc, dir) = queue.pop().unwrap();

        // Get the character at this location.
        let cell_option = grid.get_cell(&loc);
        if cell_option.is_none() {
            continue;
        }
        let char_ = cell_option.unwrap().value;

        // Add to beams.
        if !beams.contains_key(&loc) {
            beams.insert(loc, HashSet::new());
        }
        let mut set_reference = beams.get_mut(&loc).unwrap();
        if set_reference.contains(&dir) {
            // We evaluated this before, skip.
            continue;
        } else {
            // Mark that we evaluated it.
            set_reference.insert(dir);
        }

        let new_dirs = match char_ {
            '\\' => match dir {
                (1, 0) => vec![(0, 1)],
                (-1, 0) => vec![(0, -1)],
                (0, 1) => vec![(1, 0)],
                (0, -1) => vec![(-1, 0)],
                _ => panic!("Whoops"),
            },
            '/' => match dir {
                (1, 0) => vec![(0, -1)],
                (-1, 0) => vec![(0, 1)],
                (0, 1) => vec![(-1, 0)],
                (0, -1) => vec![(1, 0)],
                _ => panic!("Whoops"),
            },
            '-' => match dir {
                (1, 0) => vec![(0, 1), (0, -1)],
                (-1, 0) => vec![(0, 1), (0, -1)],
                (0, 1) => vec![(0, 1)],
                (0, -1) => vec![(0, -1)],
                _ => panic!("Whoops"),
            },
            '|' => match dir {
                (1, 0) => vec![(1, 0)],
                (-1, 0) => vec![(-1, 0)],
                (0, 1) => vec![(1, 0), (-1, 0)],
                (0, -1) => vec![(1, 0), (-1, 0)],
                _ => panic!("Whoops"),
            },
            '.' => vec![dir],
            _ => panic!("Whoops"),
        };

        for new_dir in new_dirs {
            let new_loc = GridIndex {
                x: loc.x + new_dir.0,
                y: loc.y + new_dir.1,
            };
            queue.push((new_loc, new_dir));
        }
    }
    beams.len()
}

impl Day for Day16 {
    fn get_id(&self) -> String {
        "day_16".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        solve(input, GridIndex { x: 0, y: 0 }, (0, 1)).to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let mut solutions = Vec::new();
        let line_length = input.find('\n').unwrap();

        for x in 0..line_length {
            for y in 0..line_length {
                if (x == 0 || x == line_length) || (y == 0 || y == line_length) {
                    for delta in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
                        solutions.push(solve(
                            input,
                            GridIndex {
                                x: x.try_into().unwrap(),
                                y: y.try_into().unwrap(),
                            },
                            delta,
                        ));
                    }
                }
            }
        }
        solutions.iter().max().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day16 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day16 {}.test_day_part(&'b')
    }
}
