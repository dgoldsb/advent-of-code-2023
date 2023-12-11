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

fn multi_gcd(nums: &Vec<u128>, idx: usize) -> u128 {
    if idx == nums.len() - 1 {
        return nums[idx];
    }
    let a = nums[idx];
    let b = multi_gcd(nums, idx + 1);
    return gcd(a, b);
}

pub fn lcm(nums: Vec<u128>) -> u128 {
    nums.iter().product::<u128>() / multi_gcd(&nums, 0)
}

pub fn manhattan_distance(start: &(usize, usize), end: &(usize, usize)) -> usize {
    (max(start.0, end.0) - min(start.0, end.0)) + (max(start.1, end.1) - min(start.1, end.1))
}
