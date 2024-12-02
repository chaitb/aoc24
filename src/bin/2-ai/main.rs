use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut safe_reports_part1 = 0;
    let mut safe_reports_part2 = 0;

    for line_res in stdin.lock().lines() {
        if let Ok(line) = line_res {
            let levels: Vec<i32> = line
                .trim()
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();

            if levels.len() < 2 {
                continue;
            }

            if is_safe(&levels) {
                safe_reports_part1 += 1;
                safe_reports_part2 += 1;
            } else {
                // For Part Two, try removing one level at a time
                for idx in 0..levels.len() {
                    let mut modified_levels = levels.clone();
                    modified_levels.remove(idx);
                    if modified_levels.len() < 2 {
                        continue;
                    }
                    if is_safe(&modified_levels) {
                        safe_reports_part2 += 1;
                        break;
                    }
                }
            }
        }
    }

    println!("Safe reports in Part One: {}", safe_reports_part1);
    println!("Safe reports in Part Two: {}", safe_reports_part2);
}

fn is_safe(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return false;
    }

    let differences: Vec<i32> = report.windows(2).map(|w| w[1] - w[0]).collect();

    for diff in &differences {
        let abs_diff = diff.abs();
        if abs_diff < 1 || abs_diff > 3 {
            return false;
        }
    }

    let is_increasing = differences.iter().all(|&d| d > 0);
    let is_decreasing = differences.iter().all(|&d| d < 0);

    is_increasing || is_decreasing
}
