use std::fs::read_to_string;

pub fn read_lines(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn read_file(path: &str) -> String {
    read_to_string(path).unwrap()
}
