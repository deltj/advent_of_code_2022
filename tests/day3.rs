use advent_of_code_2022::day3::*;

use std::io::BufReader;
use std::fs::File;

#[test]
fn day3_part1_example() {
    let f = File::open("data/day3_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let rucksack_vector = read_rucksack_vector(&mut reader);
    let sum = error_priority_sum(&rucksack_vector);
    assert_eq!(157, sum);
}

#[test]
fn day3_part1_actual() {
    let f = File::open("data/day3_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let rucksack_vector = read_rucksack_vector(&mut reader);
    let sum = error_priority_sum(&rucksack_vector);
    assert_eq!(7568, sum);
}