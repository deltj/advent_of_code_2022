use advent_of_code_2022::day7::*;

use std::io::{BufReader};
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
    assert_eq!(919137, sum);
}

#[test]
fn day7_part2_example() {
    let f = File::open("data/day7_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let fs = read_fs_tree(&mut reader);
    let _sz = update_sizes(fs.clone());

    let total_available_space = 70000000;
    let total_used_space = fs.borrow().size;
    let total_unused_space = total_available_space - total_used_space;

    let min_space_required = 30000000;
    let smallest_dir_size_to_delete = min_space_required - total_unused_space;

    let sdgt = smallest_dir_gt(fs.clone(), smallest_dir_size_to_delete);
    assert_eq!(24933642, sdgt);
}

#[test]
fn day7_part2_actual() {
    let f = File::open("data/day7_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let fs = read_fs_tree(&mut reader);
    let _sz = update_sizes(fs.clone());

    let total_available_space = 70000000;
    let total_used_space = fs.borrow().size;
    let total_unused_space = total_available_space - total_used_space;

    let min_space_required = 30000000;
    let smallest_dir_size_to_delete = min_space_required - total_unused_space;

    let sdgt = smallest_dir_gt(fs.clone(), smallest_dir_size_to_delete);
    assert_eq!(2877389, sdgt);
}
