use advent_of_code_2022::day1::*;

use std::io::BufReader;
use std::fs::File;

#[test]
fn day1_part1_example() {
    let f = File::open("data/day1_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let elf_calorie_vector = read_elf_calories(&mut reader);
    let max = max_calories(&elf_calorie_vector);
    assert_eq!(24000, max);
}

#[test]
fn day1_part1_actual() {
    let f = File::open("data/day1_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let elf_calorie_vector = read_elf_calories(&mut reader);
    let max = max_calories(&elf_calorie_vector);
    assert_eq!(69528, max);
}

#[test]
fn day1_part2_example() {
    let f = File::open("data/day1_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut elf_calorie_vector = read_elf_calories(&mut reader);
    let top = top_three_calories(&mut elf_calorie_vector);
    let top_sum = top.0 + top.1 + top.2;
    assert_eq!(45000, top_sum);
}

#[test]
fn day1_part2_actual() {
    let f = File::open("data/day1_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut elf_calorie_vector = read_elf_calories(&mut reader);
    let top = top_three_calories(&mut elf_calorie_vector);
    let top_sum = top.0 + top.1 + top.2;
    assert_eq!(206152, top_sum);
}