use crate::days_module::day::Day;
use helpers::replace_nth_char_ascii;

fn score(image: &String) -> usize {
    let mut lines = image.split("\n").collect::<Vec<&str>>();
    lines.reverse();
    lines
        .iter()
        .enumerate()
        .map(|(i, l)| l.chars().filter(|c| *c == 'O').count() * (i + 1))
        .sum::<usize>()
}

fn roll_rocks(image: &String, direction: char) -> String {
    let mut new_image = image.clone();
    let line_length: isize = image.find('\n').unwrap().try_into().unwrap();

    let delta: isize = match direction {
        'N' => -1 * line_length - 1,
        'S' => line_length + 1,
        'W' => -1,
        'E' => 1,
        _ => panic!("Not possible")
    };

    // For each round rock.
    for (i, _) in image.chars().enumerate().filter(|(_, c)| *c == 'O') {
        // Apply delta and switch with the last non-rock before "#".
        let mut pointer: isize = i.try_into().unwrap();
        let mut last_empty_index: isize = pointer;
        loop {
            pointer += delta;
            let pointer_usize_option = pointer.try_into();
            if pointer_usize_option.is_err() { break }
            let char_option = new_image.chars().nth(pointer_usize_option.unwrap());
            match char_option {
                Some('#') => break,
                Some('.') => last_empty_index = pointer,
                None => break,
                _ => {},
            }
        }
        replace_nth_char_ascii(&mut new_image, i, '.');
        replace_nth_char_ascii(&mut new_image, last_empty_index.try_into().unwrap(), 'O');
    }
    new_image
}

pub struct Day14 {}

impl Day for Day14 {
    fn get_id(&self) -> String {
        "day_14".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        score(&roll_rocks(input, 'N')).to_string()
    }

    fn part_b(&self, input: &String) -> String {
        // TODO: Cycle detection.
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day14 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day14 {}.test_day_part(&'b')
    }
}
