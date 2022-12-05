use advent_of_code_2022::day4::*;

use std::io::BufReader;
use std::fs::File;

#[test]
fn day4_part1_example() {
    let f = File::open("data/day4_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let sp = read_sequence_pairs(&mut reader);
    let count = count_contained_pairs(&sp);
    assert_eq!(2, count);
}

#[test]
fn day4_part1_actual() {
    let f = File::open("data/day4_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let sp = read_sequence_pairs(&mut reader);
    let count = count_contained_pairs(&sp);
    assert_eq!(582, count);
}

#[test]
fn day4_part2_example() {
    let f = File::open("data/day4_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let sp = read_sequence_pairs(&mut reader);
    let count = count_overlapped_pairs(&sp);
    assert_eq!(4, count);
}

#[test]
fn day4_part2_actual() {
    let f = File::open("data/day4_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let sp = read_sequence_pairs(&mut reader);
    let count = count_overlapped_pairs(&sp);
    assert_eq!(893, count);
}
