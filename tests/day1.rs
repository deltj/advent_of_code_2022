use advent_of_code_2022::day1::*;

use std::io::BufReader;
use std::fs::File;

#[test]
fn day1_part1_example() {
    let f = File::open("data/day1_part1_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let elf_calorie_vector = read_elf_calories(&mut reader);
    let max = max_calories(&elf_calorie_vector);
    assert_eq!(24000, max);
}

#[test]
fn day1_part1_actual() {
    let f = File::open("data/day1_part1_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let elf_calorie_vector = read_elf_calories(&mut reader);
    let max = max_calories(&elf_calorie_vector);
    assert_eq!(69528, max);
}