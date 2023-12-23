use crate::grid::cell::Cell;
use crate::grid::grid_index::GridIndex;
use core::slice::Iter;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Grid {
    cells: Vec<Cell>,
    cell_map: HashMap<GridIndex, Cell>,
}

impl Grid {
    pub fn iter(&self) -> Iter<Cell> {
        self.cells.iter()
    }

    pub fn find_index(&self, value: &char) -> Result<&GridIndex, ()> {
        for cell in &self.cells {
            if cell.value == *value {
                return Ok(&cell.index)
            }
        }
        return Err(())
    }

    pub fn get_cell(&self, index: &GridIndex) -> Option<&Cell> {
        return self.cell_map.get(index);
    }

    pub fn get_cell_by_index(&self, index: &(i32, i32)) -> Option<&Cell> {
        return self.cell_map.get(&GridIndex {
            x: index.0,
            y: index.1,
        });
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

        let cell_map = cells.iter().map(|c| (c.index, c.clone())).collect();

        return Ok(Grid { cells, cell_map });
    }
}
