use crate::days_module::day::Day;
use std::collections::HashSet;

#[derive(Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

struct Vertex {
    start: Point,
    end: Point,
}

pub struct Day18 {}

impl Day for Day18 {
    fn get_id(&self) -> String {
        "day_18".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let mut pointer = Point { x: 0, y: 0 };
        let mut vertices = Vec::new();
        let mut circumference = 0;

        for line in input.split("\n") {
            let start = pointer;

            let mut line_split = line.split(" ");
            let direction = line_split.next().unwrap().chars().next().unwrap();
            let step_count: isize = line_split.next().unwrap().parse().unwrap();

            pointer = match direction {
                'U' => Point {
                    x: pointer.x,
                    y: pointer.y + 1 * step_count,
                },
                'D' => Point {
                    x: pointer.x,
                    y: pointer.y - 1 * step_count,
                },
                'L' => Point {
                    x: pointer.x - 1 * step_count,
                    y: pointer.y,
                },
                'R' => Point {
                    x: pointer.x + 1 * step_count,
                    y: pointer.y,
                },
                _ => panic!("Whoops"),
            };
            circumference += step_count;
            vertices.push(Vertex {
                start: start,
                end: pointer,
            })
        }

        let mut first_sum = 0;
        let mut second_sum = 0;
        for vertex in vertices {
            first_sum += vertex.start.x * vertex.end.y;
            second_sum += vertex.start.y * vertex.end.x;
        }

        let shoelace_area = (first_sum - second_sum).abs() / 2;
        let area = shoelace_area + circumference / 2 + 1;
        area.to_string()
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
        Day18 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day18 {}.test_day_part(&'b')
    }
}
