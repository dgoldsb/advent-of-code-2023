use crate::days_module::day::Day;
use helpers::cube::cube::Cube;
use std::cmp::{min, Ordering};
use std::collections::HashSet;
use std::str::FromStr;

pub struct Day22 {}

fn disintegrate(cube: &Cube, all_cubes: &Vec<Cube>) -> usize {
    let mut cube_set = all_cubes
        .iter()
        .map(|c| c.clone())
        .collect::<HashSet<Cube>>();
    cube_set.remove(cube);

    loop {
        let settled_cubes = settle(cube_set.iter().map(|c| c.clone()).collect::<Vec<Cube>>())
            .iter()
            .map(|c| c.clone())
            .collect::<HashSet<Cube>>();

        let cube_set_clone = cube_set.clone();
        let fallen_cubes = cube_set_clone.difference(&settled_cubes);

        let mut escape = true;
        for fallen_cube in fallen_cubes {
            cube_set.remove(fallen_cube);
            escape = false;
        }
        if escape {
            break
        }
    }

    return all_cubes.len() - cube_set.len() - 1
}

fn supported_by<'a>(cube: &'a Cube, all_cubes: &'a Vec<Cube>) -> Vec<&'a Cube> {
    let mut supporting_cubes = Vec::new();
    for supporting in all_cubes {
        if cube.is_directly_above(supporting) {
            supporting_cubes.push(supporting);
        }
    }
    supporting_cubes
}

fn settle(mut cubes: Vec<Cube>) -> Vec<Cube> {
    // Sort the cubes by their height.
    cubes.sort_by(|this, other| {
        if this.z_range.start() < other.z_range.start() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    // Fast-forward every cube down.
    let mut settled_stack: Vec<Cube> = Vec::new();
    for cube in &cubes {
        let mut z_delta = cube.z_range.start().clone();

        for settled in &settled_stack {
            if settled.is_below(&cube) {
                z_delta = min(z_delta, cube.z_difference(settled) - 1);
            }
        }

        settled_stack.push(cube.drop_by(z_delta));
    }
    settled_stack.sort_by(|this, other| {
        if this.z_range.start() < other.z_range.start() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    settled_stack
}

impl Day for Day22 {
    fn get_id(&self) -> String {
        "day_22".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let cubes = input
            .split("\n")
            .map(|s| Cube::from_str(s).unwrap())
            .collect::<Vec<Cube>>();
        let settled_stack = settle(cubes);

        // Count the number of cubes that serve as the only support.
        let crucial_supports = settled_stack
            .iter()
            .map(|c| supported_by(c, &settled_stack))
            .filter(|v| v.len() == 1)
            .map(|v| *v.first().unwrap())
            .collect::<HashSet<&Cube>>();

        (settled_stack.len() - crucial_supports.len()).to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let cubes = input
            .split("\n")
            .map(|s| Cube::from_str(s).unwrap())
            .collect::<Vec<Cube>>();
        let settled_stack = settle(cubes);

        let crucial_supports = settled_stack
            .iter()
            .map(|c| supported_by(c, &settled_stack))
            .filter(|v| v.len() == 1)
            .map(|v| *v.first().unwrap())
            .collect::<HashSet<&Cube>>();

        crucial_supports
            .iter()
            .map(|c| disintegrate(c, &settled_stack))
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day22 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day22 {}.test_day_part(&'b')
    }
}
