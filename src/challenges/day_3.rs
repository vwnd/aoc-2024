use crate::challenges::utils::read_file;
use regex::Regex;

fn parse_mul_operation(operation: &str) -> Option<(i32, i32)> {
    let expression = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    if let Some(captures) = expression.captures(operation) {
        let num_1 = captures.get(1)?.as_str().parse::<i32>().ok()?;
        let num_2 = captures.get(2)?.as_str().parse::<i32>().ok()?;
        Some((num_1, num_2))
    } else {
        None
    }
}

pub fn solve() {
    println!("Day 3");

    // let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let input = read_file("src/data/day-3.txt");
    let expression = Regex::new(r"(?:mul\((\d+),(\d+)\))|(do\(\)|don't\(\))").unwrap();
    let matches: Vec<_> = expression.find_iter(&input).map(|m| m.as_str()).collect();

    let mut answer_1 = 0;
    let mut answer_2 = 0;
    let mut enabled = true;

    for m in matches {
        // Part 1
        if m == "do()" || m == "don't()" {
        } else {
            if let Some((num_1, num_2)) = parse_mul_operation(m) {
                let result = num_1 * num_2;
                answer_1 += result;
            }
        }

        // Part 2
        if m == "do()" {
            enabled = true;
        } else if m == "don't()" {
            enabled = false;
        } else if enabled {
            if let Some((num_1, num_2)) = parse_mul_operation(m) {
                let result = num_1 * num_2;
                answer_2 += result;
            }
        }
    }

    println!("Solution Part 1: {}", answer_1);
    println!("Solution Part 2: {}", answer_2);
}
