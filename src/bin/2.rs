// Imports
use std::io;
use std::io::prelude::*;

fn main() {
    println!("Aoc 2024 - Day 2");
    process();
}

pub fn read_lines() -> Vec<String> {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let vec = stdin_lock.lines().filter_map(|l| l.ok()).collect();

    vec
}

// ****************************************************************
// *************************** [Begin] ****************************
// ****************************************************************

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Increasing,
    Decreasing,
    Equal,
}

fn process() {
    let mut count = 0;
    for line in read_lines() {
        let levels = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        if should_count_level_damped(&levels) {
            count += 1;
        }
    }

    println!("count: {}", count);
}

fn should_count_level_damped(inp: &Vec<i32>) -> bool {
    if should_count_level(inp) {
        return true;
    }

    for j in 0..inp.len() {
        let mut inp2 = inp.clone();
        inp2.remove(j);
        if should_count_level(&inp2) {
            return true;
        }
    }

    false
}

fn should_count_level(inp: &Vec<i32>) -> bool {
    // 1. check strictly increasing or decreasing
    // 2. check (1 <= diff <= 3)

    let mut direction = Direction::Equal;

    for i in 0..inp.len() - 1 {
        let diff = inp[i + 1] - inp[i];

        match diff.abs() {
            1 | 2 | 3 => (),
            _ => return false,
        };

        let new_direction = if diff > 0 {
            Direction::Increasing
        } else {
            Direction::Decreasing
        };

        if direction == Direction::Equal {
            direction = new_direction;
        } else if direction != new_direction {
            return false;
        }
    }

    true
}
