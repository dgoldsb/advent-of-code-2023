use regex::Regex;
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

pub fn ints_from_string(input: &String) -> Vec<isize> {
    let re = Regex::new(r"([-+]?\d+)\D?").unwrap();
    re.captures_iter(input)
        .map(|c| c[1].parse().expect("Something went wrong parsing an int"))
        .collect()
}
