use advent_of_code_2022::day10::*;

use std::io::BufReader;
use std::fs::File;

#[test]
fn day10_part1_example() {
    let f = File::open("data/day10_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let program = read_program(&mut reader);

    let mut cpu = FakeComputer::new(program);

    //  Note - The fake computer is run for 19 cycles, because the problem states that we 
    //  want the value *during* the 20th cycle, not *after* it.  Running the machine for
    //  20 cycles would give the value *after* 20 cycles.
    cpu.run(19);
    assert_eq!(21, cpu.x);
    let ss20 = cpu.get_signal_strength();
    assert_eq!(420, ss20);

    cpu.run(40);
    assert_eq!(19, cpu.x);
    let ss60 = cpu.get_signal_strength();
    assert_eq!(1140, ss60);
    
    cpu.run(40);
    assert_eq!(18, cpu.x);
    let ss100 = cpu.get_signal_strength();
    assert_eq!(1800, ss100);

    cpu.run(40);
    assert_eq!(21, cpu.x);
    let ss140 = cpu.get_signal_strength();
    assert_eq!(2940, ss140);

    cpu.run(40);
    assert_eq!(16, cpu.x);
    let ss180 = cpu.get_signal_strength();
    assert_eq!(2880, ss180);

    cpu.run(40);
    assert_eq!(18, cpu.x);
    let ss240 = cpu.get_signal_strength();
    assert_eq!(3960, ss240);

    let ss_sum = ss20 + ss60 + ss100 + ss140 + ss180 + ss240;
    assert_eq!(13140, ss_sum);
}

#[test]
fn day10_part1_actual() {
    let f = File::open("data/day10_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let program = read_program(&mut reader);

    let mut cpu = FakeComputer::new(program);

    cpu.run(19);
    let ss20 = cpu.get_signal_strength();

    cpu.run(40);
    let ss60 = cpu.get_signal_strength();
    
    cpu.run(40);
    let ss100 = cpu.get_signal_strength();

    cpu.run(40);
    let ss140 = cpu.get_signal_strength();

    cpu.run(40);
    let ss180 = cpu.get_signal_strength();

    cpu.run(40);
    let ss240 = cpu.get_signal_strength();

    let ss_sum = ss20 + ss60 + ss100 + ss140 + ss180 + ss240;
    assert_eq!(14160, ss_sum);
}

#[test]
fn day10_part2_example() {
    let f = File::open("data/day10_example.txt").unwrap();
    let mut reader = BufReader::new(f);
    let program = read_program(&mut reader);
    let mut cpu = FakeComputer::new(program);
    cpu.reset();
    //cpu.render_display();
}

#[test]
fn day10_part2_actual() {
    let f = File::open("data/day10_actual.txt").unwrap();
    let mut reader = BufReader::new(f);
    let program = read_program(&mut reader);
    let mut cpu = FakeComputer::new(program);
    cpu.reset();
    //cpu.render_display();
}
