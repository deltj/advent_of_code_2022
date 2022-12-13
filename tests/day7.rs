use advent_of_code_2022::day7::*;

use std::io::{BufReader, BufRead};
use std::fs::File;

#[test]
fn day7_part1_example() {
    let f = File::open("data/day7_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let fs = read_fs_tree(&mut reader);
    let _sz = update_sizes(fs.clone());
    let sum = sum_dirs_le(fs.clone(), 100000);
    assert_eq!(95437, sum);
}

#[test]
fn day7_part1_actual() {
    let f = File::open("data/day7_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let fs = read_fs_tree(&mut reader);
    let _sz = update_sizes(fs.clone());
    let sum = sum_dirs_le(fs.clone(), 100000);
    assert_eq!(95437, sum);
}