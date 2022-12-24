use advent_of_code_2022::day11::*;

use std::io::BufReader;
use std::fs::File;

#[test]
fn day11_part1_example() {
    let f = File::open("data/day11_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut monkeys = read_monkeys(&mut reader);
    play_rounds_p1(&mut monkeys, 20);
    let monkey_business = calc_monkey_business(&monkeys);
    assert_eq!(10605, monkey_business);
}

#[test]
fn day11_part1_actual() {
    let f = File::open("data/day11_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut monkeys = read_monkeys(&mut reader);
    play_rounds_p1(&mut monkeys, 20);
    let monkey_business = calc_monkey_business(&monkeys);
    assert_eq!(66124, monkey_business);
}

#[test]
fn day11_part2_example() {
    let f = File::open("data/day11_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut monkeys = read_monkeys(&mut reader);
    play_rounds_p2(&mut monkeys, 10000);
    let monkey_business = calc_monkey_business(&monkeys);
    assert_eq!(2713310158, monkey_business);
}

#[test]
fn day11_part2_actual() {
    let f = File::open("data/day11_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut monkeys = read_monkeys(&mut reader);
    play_rounds_p2(&mut monkeys, 10000);
    let monkey_business = calc_monkey_business(&monkeys);
    assert_eq!(2713310158, monkey_business);
}
