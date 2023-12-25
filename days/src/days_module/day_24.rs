use crate::days_module::day::Day;

#[derive(Eq, PartialEq)]
struct HailStone {
    start: (isize, isize, isize),
    velocity: (isize, isize, isize),
}

impl HailStone {
    // Calculate the intersection point between two stones.
    fn intersection(&self, other: &HailStone) -> Option<(f64, f64)> {
        let dx1 = self.velocity.0 as f64;
        let dy1 = self.velocity.1 as f64;

        let dx2 = other.velocity.0 as f64;
        let dy2 = other.velocity.1 as f64;

        let determinant = dx1 * dy2 - dy1 * dx2;

        // Check if the lines are parallel (determinant close to zero).
        if determinant.abs() < 1e-10 {
            // Lines are parallel or coincident, no unique intersection point.
            return None;
        }

        let t1 = ((other.start.0 - self.start.0) as f64 * dy2
            - (other.start.1 - self.start.1) as f64 * dx2)
            / determinant;

        if t1 < 1.0 {
            return None;
        }

        let intersection_x = self.start.0 as f64 + t1 * dx1;
        let intersection_y = self.start.1 as f64 + t1 * dy1;

        Some((intersection_x, intersection_y))
    }

    // Function to create a HailStone instance from a string.
    fn from_string(input: &str) -> Result<HailStone, &'static str> {
        let parts: Vec<&str> = input.split('@').map(|s| s.trim()).collect();

        if parts.len() != 2 {
            return Err("Invalid input format");
        }

        let position_str = parts[0];
        let velocity_str = parts[1];

        let parse_triple = |s: &str| -> Result<(isize, isize, isize), &'static str> {
            let values: Result<Vec<isize>, _> = s
                .split(',')
                .map(|part| part.trim().parse().map_err(|_| "Invalid integer"))
                .collect();

            let values = values?;
            if values.len() != 3 {
                return Err("Invalid triple format");
            }

            Ok((values[0], values[1], values[2]))
        };

        let start = parse_triple(position_str)?;
        let velocity = parse_triple(velocity_str)?;

        Ok(HailStone { start, velocity })
    }
}

pub struct Day24 {}

impl Day for Day24 {
    fn get_id(&self) -> String {
        "day_24".to_string()
    }

    // TODO: 32027 too high. 27773 too high, 27995 too high...
    fn part_a(&self, input: &String) -> String {
        let stones = input
            .split("\n")
            .map(|l| HailStone::from_string(l).unwrap())
            .collect::<Vec<HailStone>>();

        let mut intersections = Vec::new();

        for (pointer, stone) in stones.iter().enumerate() {
            for other in &stones[pointer + 1..] {
                let intersection_option = stone.intersection(other);
                match intersection_option {
                    Some(c) => intersections.push(c),
                    None => {}
                }
            }
        }

        let upper;
        let lower;

        if stones.len() < 10 {
            upper = 27.0;
            lower = 7.0;
        } else {
            upper = 400000000000000.0;
            lower = 200000000000000.0;
        }

        let valid_intersections = intersections
            .iter()
            .filter(|c| c.0 <= upper)
            .filter(|c| c.1 <= upper)
            .filter(|c| c.0 >= lower)
            .filter(|c| c.1 >= lower)
            .collect::<Vec<&(f64, f64)>>();
        let valid_intersection_count = valid_intersections.len();
        valid_intersection_count.to_string()
    }

    fn part_b(&self, input: &String) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day24 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day24 {}.test_day_part(&'b')
    }
}
