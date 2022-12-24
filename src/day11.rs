use std::io::BufRead;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Clone)]
pub struct Monkey {
    pub items: Vec<i64>,
    items_inspected: i64,
    worry_op: Operator,
    worry_op_arg: i64,
    test_divisor: i64,
    dst_true: usize,
    dst_false: usize,
}

impl Monkey {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            items_inspected: 0,
            worry_op: Operator::Addition,
            worry_op_arg: 1,
            test_divisor: 1,
            dst_true: 0,
            dst_false: 0,
        }
    }
}

enum InputState {
    MonkeyNum,
    StartingItems,
    Operation,
    Test,
    DstTrue,
    DstFalse,
    Blank,
}

fn str2op(op_str: &str) -> Operator {
    match op_str.trim() {
        "+" => Operator::Addition,
        "-" => Operator::Subtraction,
        "*" => Operator::Multiplication,
        "/" => Operator::Division,
        _ => panic!("Invalid operator"),
    }
}

fn do_op(value: i64, op: Operator, arg: i64) -> i64 {
    let mut real_arg = arg;

    if arg == i64::MAX {
        real_arg = value;
    }

    match op {
        Operator::Addition => value + real_arg,
        Operator::Subtraction => value - real_arg,
        Operator::Multiplication => value * real_arg,
        Operator::Division => value / real_arg,
    }
}

//  Find a common multiple of all the test divisors.  Because they're all prime, there's no
//  need to find the LCM.
fn calc_divisor_multiple(monkeys: &Vec<Monkey>) -> i64 {
    let mut m = 1;
    for monkey in monkeys {
        m = m * monkey.test_divisor;
    }
    return m;
}

fn play_round(monkeys: &mut Vec<Monkey>, worry_divisor: i64, p: i64) {
    //  Each monkey gets a turn
    for m in 0..monkeys.len() {
        for i in 0..monkeys[m].items.len() {
            //  I don't think I could have come up with this myself.
            //  Using modulo arithmetic it is possible to divide the worry level
            //  in such a way as to preserve the division tests required for the problem.
            //  There's a great explanation here: https://www.reddit.com/r/adventofcode/comments/ziw4aq/comment/izsr5av
            let current_worry_level = monkeys[m].items[i] % p;
            let new_worry_level = do_op(current_worry_level, monkeys[m].worry_op, monkeys[m].worry_op_arg) / worry_divisor;

            //  Throw the item to the appropriate destination monkey, depending on whether
            //  the new_worry_level is evenly divisible by the test number
            //let mod_worry = new_worry_level % p;
            if new_worry_level % monkeys[m].test_divisor == 0 {
                let dst = monkeys[m].dst_true;
                monkeys[dst].items.push(new_worry_level);
            } else {
                let dst = monkeys[m].dst_false;
                monkeys[dst].items.push(new_worry_level);
            }

            monkeys[m].items_inspected += 1;
        }
        monkeys[m].items.clear();
    }
}

pub fn play_rounds_p1(monkeys: &mut Vec<Monkey>, rounds: i64) {
    let p = calc_divisor_multiple(monkeys);
    for _ in 0..rounds {
        play_round(monkeys, 3, p);
    }
}

pub fn play_rounds_p2(monkeys: &mut Vec<Monkey>, rounds: i64) {
    let p = calc_divisor_multiple(monkeys);
    for _ in 0..rounds {
        play_round(monkeys, 1, p);
    }
}

pub fn calc_monkey_business(monkeys: &Vec<Monkey>) -> i64 {
    let mut inspect_count_vector: Vec<i64> = Vec::new();

    for monkey in monkeys {
        inspect_count_vector.push(monkey.items_inspected);
    }

    inspect_count_vector.sort();
    inspect_count_vector.reverse();

    return inspect_count_vector[0] * inspect_count_vector[1];

}

pub fn read_monkeys(reader: &mut dyn BufRead) -> Vec<Monkey> {
    let mut input_state: InputState = InputState::MonkeyNum;
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey: Monkey = Monkey::new();

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let trimmed_line = line.trim();

        match input_state {
            InputState::MonkeyNum => {
                let tokens = trimmed_line.split(" ").collect::<Vec<&str>>();
                //let monkey_num_str = tokens[1].replace(":", "");
                //let _monkey_num = monkey_num_str.parse::<i64>().unwrap();

                monkey.items.clear();

                input_state = InputState::StartingItems;
            },
            InputState::StartingItems => {
                let tokens = trimmed_line.split(":").collect::<Vec<&str>>();
                let item_csv = tokens[1];
                let item_str_vec = item_csv.split(",").collect::<Vec<&str>>();
                for item_str in item_str_vec {
                    monkey.items.push(item_str.trim().parse::<i64>().unwrap());
                }

                input_state = InputState::Operation;
            },
            InputState::Operation => {
                let tokens = trimmed_line.split(" ").collect::<Vec<&str>>();
                let op_str = tokens[4];
                let arg_str = tokens[5];
                monkey.worry_op = str2op(op_str);

                if arg_str == "old" {
                    monkey.worry_op_arg = i64::MAX;
                } else {
                    monkey.worry_op_arg = tokens[5].parse::<i64>().unwrap();
                }

                input_state = InputState::Test;
            },
            InputState::Test => {
                let tokens = trimmed_line.split(" ").collect::<Vec<&str>>();
                let test_val_str = tokens[3];
                monkey.test_divisor = test_val_str.parse::<i64>().unwrap();

                input_state = InputState::DstTrue;
            },
            InputState::DstTrue => {
                let tokens = trimmed_line.split(" ").collect::<Vec<&str>>();
                let dst_str = tokens[5];
                monkey.dst_true = dst_str.parse::<usize>().unwrap();

                input_state = InputState::DstFalse;
            },
            InputState::DstFalse => {
                let tokens = trimmed_line.split(" ").collect::<Vec<&str>>();
                let dst_str = tokens[5];
                monkey.dst_false = dst_str.parse::<usize>().unwrap();

                input_state = InputState::Blank;

                monkeys.push(monkey.clone());
            },
            InputState::Blank => {
                input_state = InputState::MonkeyNum;
            },
        }
    }

    return monkeys;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let val = 3;
        let op = Operator::Addition;
        let arg = 3;
        let result = do_op(val, op, arg);
        assert_eq!(6, result);
    }

    #[test]
    fn subtraction() {
        let val = 3;
        let op = Operator::Subtraction;
        let arg = 3;
        let result = do_op(val, op, arg);
        assert_eq!(0, result);
    }

    #[test]
    fn multiplication() {
        let val = 3;
        let op = Operator::Multiplication;
        let arg = 3;
        let result = do_op(val, op, arg);
        assert_eq!(9, result);
    }

    #[test]
    fn division() {
        let val = 3;
        let op = Operator::Division;
        let arg = 3;
        let result = do_op(val, op, arg);
        assert_eq!(1, result);
    }

    #[test]
    fn how_does_rounding_work() {
        let worry = 1501;
        let adjusted_worry = worry / 3;
        assert_eq!(500, adjusted_worry);

        let worry = 71;
        let adjusted_worry = worry / 3;
        assert_eq!(23, adjusted_worry);
    }

    #[test]
    fn read_one_monkey() {
        let input = "Monkey 0:
            Starting items: 79, 98
            Operation: new = old * 19
            Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3";
        let mut buf = input.as_bytes();
        let monkeys = read_monkeys(&mut buf);
        assert_eq!(1, monkeys.len());
        assert_eq!(2, monkeys[0].items.len());
        assert_eq!(79, monkeys[0].items[0]);
        assert_eq!(98, monkeys[0].items[1]);
        assert_eq!(Operator::Multiplication, monkeys[0].worry_op);
        assert_eq!(19, monkeys[0].worry_op_arg);
        assert_eq!(23, monkeys[0].test_divisor);
        assert_eq!(2, monkeys[0].dst_true);
        assert_eq!(3, monkeys[0].dst_false);
    }

    #[test]
    fn read_two_monkeys() {
        let input = "Monkey 0:
            Starting items: 79, 98
            Operation: new = old * 19
            Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3
            
            Monkey 1:
            Starting items: 54, 65, 75, 74
            Operation: new = old + 6
            Test: divisible by 19
                If true: throw to monkey 2
                If false: throw to monkey 0";
        let mut buf = input.as_bytes();
        let monkeys = read_monkeys(&mut buf);
        assert_eq!(2, monkeys.len());
        assert_eq!(2, monkeys[0].items.len());
        assert_eq!(79, monkeys[0].items[0]);
        assert_eq!(98, monkeys[0].items[1]);
        assert_eq!(Operator::Multiplication, monkeys[0].worry_op);
        assert_eq!(19, monkeys[0].worry_op_arg);
        assert_eq!(23, monkeys[0].test_divisor);
        assert_eq!(2, monkeys[0].dst_true);
        assert_eq!(3, monkeys[0].dst_false);

        assert_eq!(4, monkeys[1].items.len());
        assert_eq!(54, monkeys[1].items[0]);
        assert_eq!(65, monkeys[1].items[1]);
        assert_eq!(75, monkeys[1].items[2]);
        assert_eq!(74, monkeys[1].items[3]);
        assert_eq!(Operator::Addition, monkeys[1].worry_op);
        assert_eq!(6, monkeys[1].worry_op_arg);
        assert_eq!(19, monkeys[1].test_divisor);
        assert_eq!(2, monkeys[1].dst_true);
        assert_eq!(0, monkeys[1].dst_false);
    }
}