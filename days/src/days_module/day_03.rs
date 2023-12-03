use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::GridIndex;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Day03 {}

// Extract numbers from the engine schematic, plus their Moore's neighborhood.
fn extract_numbers(grid: &Grid) -> Vec<(i32, HashSet<GridIndex>)> {
    let mut result = Vec::new();

    let mut digit_buffer = Vec::new();
    let mut index_buffer = HashSet::new();

    for cell in grid.iter() {
        // If index is a new row, then flush buffer if any.
        // If not a digit, flush buffer.
        if (cell.index.y == 0 || !cell.value.is_digit(10)) && !index_buffer.is_empty() {
            let moore_buffer = index_buffer
                .iter()
                .map(|cell: &GridIndex| cell.moore_neighborhood())
                .flatten()
                .collect::<HashSet<GridIndex>>();
            let number = digit_buffer
                .iter()
                .cloned()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();

            result.push((number, moore_buffer));

            index_buffer.clear();
            digit_buffer.clear();
        }

        // If it is a digit, add to buffer.
        if cell.value.is_digit(10) {
            digit_buffer.push(cell.value);
            index_buffer.insert(cell.index);
        }
    }

    result
}

fn extract_part_numbers(input: &String) -> Vec<i32> {
    let grid = Grid::from_str(input).unwrap();
    let mut result = Vec::new();

    let symbol_grid_indices = grid
        .iter()
        .filter(|cell| cell.is_symbol())
        .map(|cell| cell.index)
        .collect::<HashSet<GridIndex>>();

    for (number, neighborhood) in extract_numbers(&grid) {
        if !neighborhood.is_disjoint(&symbol_grid_indices) {
            result.push(number)
        }
    }

    result
}

fn extract_gear_ratios(input: &String) -> Vec<i32> {
    let grid = Grid::from_str(input).unwrap();
    let mut result = Vec::new();

    // First get indices of all gears.
    let gears = grid
        .iter()
        .filter(|cell| cell.value == '*')
        .map(|cell| cell.index)
        .collect::<HashSet<GridIndex>>();

    // Also we need all numbers plus their neighborhood.
    let extracted_numbers = extract_numbers(&grid);

    // For each gear, if there are two matches we return the product.
    for gear in gears {
        let filtered_numbers = extracted_numbers
            .iter()
            .filter(|t| t.1.contains(&gear))
            .map(|t| t.0)
            .collect::<Vec<i32>>();
        if filtered_numbers.len() == 2 {
            result.push(filtered_numbers.iter().product());
        }
    }

    result
}

impl Day for Day03 {
    fn get_id(&self) -> String {
        "day_03".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        extract_part_numbers(input).iter().sum::<i32>().to_string()
    }

    fn part_b(&self, input: &String) -> String {
        extract_gear_ratios(input).iter().sum::<i32>().to_string()
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
