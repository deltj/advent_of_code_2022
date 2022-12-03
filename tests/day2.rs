use advent_of_code_2022::day2::*;

use std::io::BufReader;
use std::fs::File;

#[test]
fn day2_part1_example() {
    let f = File::open("data/day2_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let strategy_vector = read_strategy1_vector(&mut reader);
    let round_vector = solve_type1(&strategy_vector);
    let score = total_score(&round_vector);
    assert_eq!(15, score);
}

#[test]
fn day2_part1_actual() {
    let f = File::open("data/day2_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let strategy_vector = read_strategy1_vector(&mut reader);
    let round_vector = solve_type1(&strategy_vector);
    let score = total_score(&round_vector);
    assert_eq!(13009, score);
}

#[test]
fn day2_part2_example() {
    let f = File::open("data/day2_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let strategy_vector = read_strategy2_vector(&mut reader);
    let round_vector = solve_type2(&strategy_vector);
    let score = total_score(&round_vector);
    assert_eq!(12, score);
}
#[test]
fn day2_part2_actual() {
    let f = File::open("data/day2_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let strategy_vector = read_strategy2_vector(&mut reader);
    let round_vector = solve_type2(&strategy_vector);
    let score = total_score(&round_vector);
    assert_eq!(10398, score);
}
