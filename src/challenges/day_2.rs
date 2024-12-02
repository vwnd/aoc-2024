use super::utils::read_lines;

fn is_all_incresing_or_decresing(report: &Vec<i32>) -> bool {
    let mut increasing = false;
    let mut decreasing: bool = false;

    for i in 1..report.len() {
        let previous = report[i - 1];
        let current = report[i];

        if previous < current {
            increasing = true;
        } else if previous > current {
            decreasing = true;
        }

        if increasing && decreasing {
            return false;
        }
    }

    return true;
}

fn has_low_variance(report: &Vec<i32>) -> bool {
    for i in 1..report.len() {
        let previous = report[i - 1];
        let current = report[i];

        let difference: i32 = (previous - current).abs();

        if difference < 1 || difference > 3 {
            return false;
        }
    }

    return true;
}

fn passes_problem_dampener(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let mut temp = report.clone();
        temp.remove(i);
        if is_all_incresing_or_decresing(&temp) && has_low_variance(&temp) {
            return true;
        }
    }

    return false;
}

pub fn solve() {
    println!("Day 2");

    let path = "src/data/day-2.txt";

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in read_lines(path) {
        let report: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        reports.push(report);
    }

    let mut safe_reports: i32 = 0;

    for report in reports {
        if is_all_incresing_or_decresing(&report) && has_low_variance(&report) {
            safe_reports += 1;
        } else if passes_problem_dampener(&report) {
            safe_reports += 1;
        }
    }

    println!("Solution Part 1 - Safe Reports: {}", safe_reports);
}
