use crate::challenges::utils::read_file;
use regex::Regex;

pub fn solve() {
    println!("Day 3");

    // let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let input = read_file("src/data/day-3.txt");
    let expression = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches: Vec<_> = expression.find_iter(&input).map(|m| m.as_str()).collect();

    let mut answer = 0;
    for m in matches {
        let values: Vec<_> = m
            .trim_start_matches("mul(")
            .trim_end_matches(")")
            .split(",")
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        answer += values[0] * values[1];
    }

    println!("Solution Part 1: {}", answer);
}
