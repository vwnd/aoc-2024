use std::fs::read_to_string;

fn read_lines(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn solve() {
    // print the solution to the console
    println!("Day 1");

    // Define data to the input file
    let path = "src/data/day-1.txt";

    // Initialize vectors to store the left and right ID's
    let mut left_ids = Vec::new();
    let mut right_ids = Vec::new();

    // Open the file and read it line by line
    for line in read_lines(path) {
        // Split the line into two parts
        let parts: Vec<&str> = line.split("   ").collect();
        let left_id = parts[0].to_string();
        let right_id = parts[1].to_string();

        // Push the left and right ID's into their respective vectors
        left_ids.push(left_id);
        right_ids.push(right_id);
    }

    // Sort the left and right ID's
    left_ids.sort();
    right_ids.sort();

    // throw if the arrays are not the same length
    if left_ids.len() != right_ids.len() {
        panic!("The left and right ID's are not the same length");
    }

    // create a new array to store the differences
    let mut differences = Vec::new();

    // iterate through the left and right ID's and calculate the differences
    for i in 0..left_ids.len() {
        let left_id = &left_ids[i];
        let right_id = &right_ids[i];

        // cast the left and right ID's to i32
        let left_id: i32 = left_id.parse().unwrap();
        let right_id: i32 = right_id.parse().unwrap();

        // calculate the difference between the left and right ID's and take the absolute value
        let difference = (left_id - right_id).abs();

        // push the difference into the differences array
        differences.push(difference);
    }

    // calculate the sum of the differences
    let sum: i32 = differences.iter().sum();
    println!("Solution - Sum: {}", sum);
}
