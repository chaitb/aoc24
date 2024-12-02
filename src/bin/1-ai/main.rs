use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        let mut parts = line.trim().split_whitespace();
        let left_number: i64 = parts.next().unwrap().parse().unwrap();
        let right_number: i64 = parts.next().unwrap().parse().unwrap();
        left_list.push(left_number);
        right_list.push(right_number);
    }

    // Part One: Total distance calculation
    let mut left_sorted = left_list.clone();
    let mut right_sorted = right_list.clone();
    left_sorted.sort();
    right_sorted.sort();

    let total_distance: i64 = left_sorted
        .iter()
        .zip(right_sorted.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("{}", total_distance);

    // Part Two: Similarity score calculation
    let mut frequency_map = HashMap::new();
    for &number in right_list.iter() {
        *frequency_map.entry(number).or_insert(0) += 1;
    }

    let similarity_score: i64 = left_list
        .iter()
        .map(|&number| number * frequency_map.get(&number).unwrap_or(&0))
        .sum();

    println!("{}", similarity_score);
}
