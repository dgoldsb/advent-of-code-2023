use crate::days_module::day::Day;
use helpers::grid::cell::Cell;
use helpers::grid::grid::Grid;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Day03 {}

fn is_part_number(cell: &Cell, grid: &Grid) -> bool {
    for index in cell.index.moore_neighborhood() {
        let cell_option = grid.get_cell(&index);
        if cell_option.is_some() && cell_option.unwrap().is_symbol() {
            return true;
        }
    }
    false
}

fn extract_part_numbers(input: &String) -> Vec<usize> {
    let grid = Grid::from_str(input).unwrap();

    let mut result = Vec::new();
    let mut cell_buffer: Vec<&Cell> = Vec::new();

    for cell in grid.iter() {
        // End of current number.
        if (cell.index.y == 0 || !cell.is_digit()) && !cell_buffer.is_empty() {
            let number = cell_buffer
                .iter()
                .map(|cell| cell.value)
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            for cell in &cell_buffer {
                if is_part_number(cell, &grid) {
                    result.push(number);
                    break;
                }
            }

            cell_buffer.clear();
        }

        // Extend buffer of current number.
        if cell.is_digit() {
            cell_buffer.push(cell);
        }
    }

    return result;
}

fn extract_gear_ratios(input: &String) -> Vec<usize> {
    let grid = Grid::from_str(input).unwrap();

    let mut cell_buffer: Vec<&Cell> = Vec::new();
    let mut gear_multiply_map = HashMap::new();
    let mut gear_count_map = HashMap::new();

    for cell in grid.iter() {
        // End of current number.
        if (cell.index.y == 0 || !cell.is_digit()) && !cell_buffer.is_empty() {
            let number = cell_buffer
                .iter()
                .map(|cell| cell.value)
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            // Avoid duplicate counts.
            let mut seen = Vec::new();
            for cell in &cell_buffer {
                for neighbour_index in cell.index.moore_neighborhood() {
                    if grid.get_cell(&neighbour_index).unwrap_or(cell).is('*') {
                        if seen.contains(&neighbour_index) {
                            continue;
                        }

                        let new_value =
                            gear_multiply_map.get(&neighbour_index).unwrap_or(&1) * number;
                        gear_multiply_map.insert(neighbour_index, new_value);

                        let new_count = gear_count_map.get(&neighbour_index).unwrap_or(&0) + 1;
                        gear_count_map.insert(neighbour_index, new_count);

                        seen.push(neighbour_index);
                    }
                }
            }

            cell_buffer.clear();
        }

        // Extend buffer of current number.
        if cell.is_digit() {
            cell_buffer.push(cell);
        }
    }

    gear_multiply_map
        .iter()
        .filter(|(k, _)| *gear_count_map.get(k).unwrap() == 2)
        .map(|(_, v)| v.to_owned())
        .collect::<Vec<usize>>()
}

impl Day for Day03 {
    fn get_id(&self) -> String {
        "day_03".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        extract_part_numbers(input)
            .iter()
            .sum::<usize>()
            .to_string()
    }

    fn part_b(&self, input: &String) -> String {
        extract_gear_ratios(input).iter().sum::<usize>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day03 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day03 {}.test_day_part(&'b')
    }
}
