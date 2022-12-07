use advent_of_code_2022::day6::*;

use std::io::{BufReader, BufRead};
use std::fs::File;

#[test]
fn day6_part1_actual() {
    let f = File::open("data/day6_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    let _num_bytes = reader.read_line(&mut buf);
    let sop = find_sop(&buf, 4);
    assert_eq!(1929, sop);
}

#[test]
#[ignore] // This test is super slow!
fn day6_part2_actual() {
    let f = File::open("data/day6_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    let _num_bytes = reader.read_line(&mut buf);
    let sop = find_sop(&buf, 14);
    assert_eq!(3298, sop);
}