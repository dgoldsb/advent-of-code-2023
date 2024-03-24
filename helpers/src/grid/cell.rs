use crate::grid::grid_index::GridIndex;

#[derive(Clone)]
pub struct Cell {
    pub index: GridIndex,
    pub value: char,
}

impl Cell {
    pub fn is_symbol(&self) -> bool {
        // `.` is used in AoC for empty space, typically.
        self.value != '.' && !self.value.is_digit(10) && !self.value.is_alphabetic()
    }

    pub fn is_digit(&self) -> bool {
        self.value.is_digit(10)
    }

    pub fn is(&self, reference: char) -> bool {
        self.value == reference
    }
}
