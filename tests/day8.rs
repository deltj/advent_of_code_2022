use advent_of_code_2022::day8::*;

use std::io::{BufReader};
use std::fs::File;

#[test]
fn day7_part1_example() {
    let f = File::open("data/day8_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let trees = read_tree_heights(&mut reader);
    let visible_trees = count_visible_trees(&trees);
    assert_eq!(21, visible_trees);
}

#[test]
fn day7_part1_actual() {
    let f = File::open("data/day8_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let trees = read_tree_heights(&mut reader);
    let visible_trees = count_visible_trees(&trees);
    assert_eq!(1789, visible_trees);
}

#[test]
fn day7_part2_example() {
    let f = File::open("data/day8_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let trees = read_tree_heights(&mut reader);
    let highest_score = highest_scenic_score(&trees);
    assert_eq!(8, highest_score);
}

#[test]
fn day7_part2_actual() {
    let f = File::open("data/day8_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let trees = read_tree_heights(&mut reader);
    let highest_score = highest_scenic_score(&trees);
    assert_eq!(314820, highest_score);
}
