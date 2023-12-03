use crate::grid::cell::Cell;
use crate::grid::grid_index::GridIndex;
use core::slice::Iter;
use std::str::FromStr;

pub struct Grid {
    cells: Vec<Cell>,
}

impl Grid {
    pub fn iter(&self) -> Iter<Cell> {
        self.cells.iter()
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(input: &str) -> Result<Grid, Self::Err> {
        let mut cells = Vec::new();

        let mut row_index = 0;
        for line in input.split("\n") {
            let mut column_index = 0;
            for char_ in line.chars() {
                cells.push(Cell {
                    index: GridIndex {
                        x: row_index,
                        y: column_index,
                    },
                    value: char_,
                });
                column_index += 1;
            }
            row_index += 1;
        }

        return Ok(Grid { cells });
    }
}
