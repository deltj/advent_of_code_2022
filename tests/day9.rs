use advent_of_code_2022::day9::*;

use std::collections::HashSet;
use std::io::BufReader;
use std::fs::File;

#[test]
fn day9_part1_example() {
    let f = File::open("data/day9_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let movements = read_movements(&mut reader);
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    let mut history: HashSet<(i32, i32)> = HashSet::new();
    process_movements(&mut head_pos, &mut tail_pos, &movements, &mut history);
    
    let visited_positions = history.len();
    assert_eq!(13, visited_positions);
}

#[test]
fn day9_part1_actual() {
    let f = File::open("data/day9_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let movements = read_movements(&mut reader);
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    let mut history: HashSet<(i32, i32)> = HashSet::new();
    process_movements(&mut head_pos, &mut tail_pos, &movements, &mut history);
    
    let visited_positions = history.len();
    assert_eq!(6030, visited_positions);
}

#[test]
fn day9_part2_example() {
    let f = File::open("data/day8_example.txt").unwrap();
    let mut _reader = BufReader::new(f);
}

#[test]
fn day9_part2_actual() {
    let f = File::open("data/day9_actual.txt").unwrap();
    let mut _reader = BufReader::new(f);
}
