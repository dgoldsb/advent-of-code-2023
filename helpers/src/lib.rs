pub mod cube;
pub mod cycle;
pub mod grid;
use std::cmp::{max, min};
use std::fs;

pub fn read_file(day: String) -> Result<String, String> {
    let file_path = format!("../input/{}.txt", day);

    if let Ok(metadata) = fs::metadata(&file_path) {
        if metadata.is_file() {
            match fs::read_to_string(&file_path) {
                Ok(contents) => Ok(contents),
                Err(err) => Err(format!("Error reading file: {}", err)),
            }
        } else {
            Err("The path does not point to a file.".to_string())
        }
    } else {
        Err("The file does not exist.".to_string())
    }
}

pub fn ints_from_string(input: &str) -> Vec<isize> {
    input
        .split(" ")
        .map(|s| s.parse())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect::<Vec<isize>>()
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

pub fn replace_nth_char_ascii(s: &mut str, idx: usize, newchar: char) {
    let s_bytes: &mut [u8] = unsafe { s.as_bytes_mut() };
    assert!(idx < s_bytes.len());
    assert!(s_bytes[idx].is_ascii());
    assert!(newchar.is_ascii());
    // we've made sure this is safe.
    s_bytes[idx] = newchar as u8;
}

pub fn transpose_string(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        // If there are no lines, there's nothing to transpose
        return String::new();
    }

    let num_lines = lines.len();
    let max_line_length = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let mut transposed_lines = Vec::with_capacity(max_line_length);

    for i in 0..max_line_length {
        let transposed_line: String = (0..num_lines)
            .map(|j| lines[j].chars().nth(i).unwrap_or(' '))
            .collect();

        transposed_lines.push(transposed_line);
    }

    transposed_lines.join("\n")
}

fn base_lcm(a: u128, b: u128) -> u128 {
    if a == 0 || b == 0 {
        0
    } else {
        a * b / gcd(a, b)
    }
}

pub fn lcm(numbers: &Vec<u128>) -> u128 {
    numbers.iter().fold(1, |acc, &x| base_lcm(acc, x))
}

pub fn manhattan_distance(start: &(usize, usize), end: &(usize, usize)) -> usize {
    (max(start.0, end.0) - min(start.0, end.0)) + (max(start.1, end.1) - min(start.1, end.1))
}
