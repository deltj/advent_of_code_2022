use advent_of_code_2022::day5::*;

use std::io::BufReader;
use std::fs::File;

#[test]
fn day5_part1_example() {
    let f = File::open("data/day5_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let (mut stacks, moves) = read_stacks_and_procedure(&mut reader);
    process_moves(&mut stacks, &moves);
    let top = top_crate_str(&stacks);
    assert_eq!("CMZ", top);
}

#[test]
fn day5_part1_actual() {
    let f = File::open("data/day5_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let (mut stacks, moves) = read_stacks_and_procedure(&mut reader);
    process_moves(&mut stacks, &moves);
    let top = top_crate_str(&stacks);
    assert_eq!("WHTLRMZRC", top);
}

#[test]
fn day5_part2_example() {
    let f = File::open("data/day5_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let (mut stacks, moves) = read_stacks_and_procedure(&mut reader);
    process_moves2(&mut stacks, &moves);
    let top = top_crate_str(&stacks);
    assert_eq!("MCD", top);
}

#[test]
fn day5_part2_actual() {
    let f = File::open("data/day5_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let (mut stacks, moves) = read_stacks_and_procedure(&mut reader);
    process_moves2(&mut stacks, &moves);
    let top = top_crate_str(&stacks);
    assert_eq!("GMPMLWNMG", top);
}