use std::collections::HashSet;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct GridIndex {
    pub x: i32,
    pub y: i32,
}

impl GridIndex {
    pub fn moore_neighborhood(&self) -> HashSet<GridIndex> {
        let mut result = HashSet::new();

        for x_delta in -1..=1 {
            for y_delta in -1..=1 {
                if x_delta == y_delta && x_delta == 0 {
                    continue;
                }
                result.insert(GridIndex {
                    x: self.x + x_delta,
                    y: self.y + y_delta,
                });
            }
        }

        result
    }
}