// Imports
use std::io;
use std::io::prelude::*;

// input: 2 lists of integers
// each row has 2 integers separated by whitespace
fn main() {
    println!("Aoc 2024 - Day 1");
    collect_input();
}

fn collect_input() {
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for line in read_lines() {
        let mut parts = line.split_whitespace();
        let num1 = parts.next().unwrap().parse::<i32>().unwrap();
        let num2 = parts.next().unwrap().parse::<i32>().unwrap();

        list1.push(num1);
        list2.push(num2);
    }

    // sort both lists
    list1.sort();
    list2.sort();

    // println!("{:?}", list1);
    // println!("{:?}", list2);

    // loop together
    let mut sum = 0;
    for (num1, num2) in list1.iter().zip(list2.iter()) {
        let delta = (num1 - num2).abs();
        // println!("delta {}", delta);
        sum += delta;
    }

    println!("sum: {}", sum);

    let mut i2 = 0;
    let mut score = 0;
    for n1 in list1 {
        let mut matches = 0;
        while i2 < list2.len() && list2[i2] < n1 {
            i2 += 1;
        }
        while i2 < list2.len() && list2[i2] == n1 {
            matches += 1;
            i2 += 1;
        }
        score += matches * n1;
    }

    println!("score: {}", score);
}

pub fn read_lines() -> Vec<String> {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let vec = stdin_lock.lines().filter_map(|l| l.ok()).collect();

    vec
}
