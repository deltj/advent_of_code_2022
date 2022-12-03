use advent_of_code_2022::*;

use std::io::BufReader;
use std::fs::File;

#[test]
fn day1_part1_example() {
    let f = File::open("data/day1_part1_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let result = day1_part1(&mut reader);
    assert_eq!(24000, result);
}

#[test]
fn day1_part1_actual() {
    let f = File::open("data/day1_part1_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let result = day1_part1(&mut reader);
    assert_eq!(24000, result);
}