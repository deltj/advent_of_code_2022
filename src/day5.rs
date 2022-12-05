use std::{io::BufRead, collections::VecDeque};

#[derive(Debug,PartialEq)]
pub struct StackMove {
    src: usize,
    dst: usize,
    count: usize,
}

/// Read initial stack configuration and vector of moves from the input
pub fn read_stacks_and_procedure(reader: &mut dyn BufRead) -> (Vec<VecDeque<char>>, Vec<StackMove>) {
    let mut stack_vector: Vec<VecDeque<char>> = Vec::new();
    let mut move_vector: Vec<StackMove> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let n = line.len(); // n=length of the line, including spaces
        let s = (n/4)+1;    // s=number of stacks

        if stack_vector.len() == 0 {
            // Initialize stacks
            for _i in 0..s {
                let stack: VecDeque<char> = VecDeque::new();
                stack_vector.push(stack);
            }
        }

        // Check whether we're reading stack or procedure input
        if line.contains('[') {
            // All stack lines should contain at least one bracket

            for i in 0..s {
                let c = line.chars().nth(1 + (i * 4)).unwrap();
                if ('A'..='Z').contains(&c) {
                    stack_vector[i].push_front(c);
                }
            }
        } else if line.contains("move") {
            // All procedure lines should contain the word move

            let tok = line.split(" ").collect::<Vec<&str>>();
            let sm: StackMove = StackMove {
                src: tok[3].parse::<usize>().unwrap() - 1,
                dst: tok[5].parse::<usize>().unwrap() - 1,
                count:     tok[1].parse::<usize>().unwrap(),
            };
            move_vector.push(sm);
        }
    }

    return (stack_vector, move_vector);
}

/// Process a move using the part 1 method
fn process_move(stacks: &mut Vec<VecDeque<char>>, m: &StackMove) {
    for _c in 0..m.count {
        let tmp = stacks[m.src].pop_back().unwrap();
        stacks[m.dst].push_back(tmp);
    }
}

/// Process a move using the part 2 method
fn process_move2(stacks: &mut Vec<VecDeque<char>>, m: &StackMove) {
    let mut tmpvec: VecDeque<char> = VecDeque::new();

    for _c in 0..m.count {
        let tmp = stacks[m.src].pop_back().unwrap();
        tmpvec.push_front(tmp);
    }

    for c in tmpvec {
        stacks[m.dst].push_back(c);
    }
}

/// Process all moves using the part 1 method
pub fn process_moves(stacks: &mut Vec<VecDeque<char>>, moves: &Vec<StackMove>) {
    for m in moves {
        process_move(stacks, &m);
    }
}

/// Process all moves using the part 2 method
pub fn process_moves2(stacks: &mut Vec<VecDeque<char>>, moves: &Vec<StackMove>) {
    for m in moves {
        process_move2(stacks, &m);
    }
}

/// Make a string from the characters at the top of each stack
pub fn top_crate_str(stacks: &Vec<VecDeque<char>>) -> String {
    let mut s: String = String::from("");

    for stack in stacks {
        s.push(*stack.back().unwrap());
    }
    return s;
}

#[cfg(test)]
mod tests {
    use std::{fs::read};

    use super::*;

    #[test]
    fn read_stacks_and_procedure_test() {
        let input = concat!("    [D]    \n",
                            "[N] [C]    \n",
                            "[Z] [M] [P]\n",
                            " 1   2   3 \n",
                            "           \n",
                            "move 1 from 2 to 1\n",
                            "move 3 from 1 to 3\n",
                            "move 2 from 2 to 1\n",
                            "move 1 from 1 to 2\n");
        let mut buf = input.as_bytes();
        let (mut stacks, moves) = read_stacks_and_procedure(&mut buf);
        assert_eq!(3, stacks.len());

        assert_eq!(2, stacks[0].len());
        assert_eq!('N', stacks[0][1]);
        assert_eq!('Z', stacks[0][0]);

        assert_eq!(3, stacks[1].len());
        assert_eq!('D', stacks[1][2]);
        assert_eq!('C', stacks[1][1]);
        assert_eq!('M', stacks[1][0]);

        assert_eq!(1, stacks[2].len());
        assert_eq!('P', stacks[2][0]);

        assert_eq!(4, moves.len());
        assert_eq!(StackMove{ src: 1, dst: 0, count: 1 }, moves[0]);
        assert_eq!(StackMove{ src: 0, dst: 2, count: 3 }, moves[1]);
        assert_eq!(StackMove{ src: 1, dst: 0, count: 2 }, moves[2]);
        assert_eq!(StackMove{ src: 0, dst: 1, count: 1 }, moves[3]);

        process_move(&mut stacks, &moves[0]);

        //  Expected stacks after move:
        // [D]        
        // [N] [C]    
        // [Z] [M] [P]
        //  1   2   3 
        assert_eq!(3, stacks[0].len());
        assert_eq!('D', stacks[0][2]);
        assert_eq!('N', stacks[0][1]);
        assert_eq!('Z', stacks[0][0]);

        assert_eq!(2, stacks[1].len());
        assert_eq!('C', stacks[1][1]);
        assert_eq!('M', stacks[1][0]);

        assert_eq!(1, stacks[2].len());
        assert_eq!('P', stacks[2][0]);

        let top = top_crate_str(&stacks);
        assert_eq!("DCP", top);

        process_move(&mut stacks, &moves[1]);

        //  Expected stacks after move:
        //         [Z]
        //         [N]
        //     [C] [D]
        //     [M] [P]
        //  1   2   3
        assert_eq!(0, stacks[0].len());

        assert_eq!(2, stacks[1].len());
        assert_eq!('C', stacks[1][1]);
        assert_eq!('M', stacks[1][0]);

        assert_eq!(4, stacks[2].len());
        assert_eq!('Z', stacks[2][3]);
        assert_eq!('N', stacks[2][2]);
        assert_eq!('D', stacks[2][1]);
        assert_eq!('P', stacks[2][0]);

    }

    #[test]
    fn read_stacks_and_procedure_test2() {
        let input = concat!("    [D]    \n",
                            "[N] [C]    \n",
                            "[Z] [M] [P]\n",
                            " 1   2   3 \n",
                            "           \n",
                            "move 1 from 2 to 1\n",
                            "move 3 from 1 to 3\n",
                            "move 2 from 2 to 1\n",
                            "move 1 from 1 to 2\n");
        let mut buf = input.as_bytes();
        let (mut stacks, moves) = read_stacks_and_procedure(&mut buf);
        assert_eq!(3, stacks.len());

        assert_eq!(2, stacks[0].len());
        assert_eq!('N', stacks[0][1]);
        assert_eq!('Z', stacks[0][0]);

        assert_eq!(3, stacks[1].len());
        assert_eq!('D', stacks[1][2]);
        assert_eq!('C', stacks[1][1]);
        assert_eq!('M', stacks[1][0]);

        assert_eq!(1, stacks[2].len());
        assert_eq!('P', stacks[2][0]);

        assert_eq!(4, moves.len());
        assert_eq!(StackMove{ src: 1, dst: 0, count: 1 }, moves[0]);
        assert_eq!(StackMove{ src: 0, dst: 2, count: 3 }, moves[1]);
        assert_eq!(StackMove{ src: 1, dst: 0, count: 2 }, moves[2]);
        assert_eq!(StackMove{ src: 0, dst: 1, count: 1 }, moves[3]);

        process_move2(&mut stacks, &moves[0]);

        //  Expected stacks after move:
        // [D]        
        // [N] [C]    
        // [Z] [M] [P]
        //  1   2   3 
        assert_eq!(3, stacks[0].len());
        assert_eq!('D', stacks[0][2]);
        assert_eq!('N', stacks[0][1]);
        assert_eq!('Z', stacks[0][0]);

        assert_eq!(2, stacks[1].len());
        assert_eq!('C', stacks[1][1]);
        assert_eq!('M', stacks[1][0]);

        assert_eq!(1, stacks[2].len());
        assert_eq!('P', stacks[2][0]);

        let top = top_crate_str(&stacks);
        assert_eq!("DCP", top);

        process_move2(&mut stacks, &moves[1]);

        //  Expected stacks after move:
        //         [D]
        //         [N]
        //     [C] [Z]
        //     [M] [P]
        //  1   2   3
        assert_eq!(0, stacks[0].len());

        assert_eq!(2, stacks[1].len());
        assert_eq!('C', stacks[1][1]);
        assert_eq!('M', stacks[1][0]);

        assert_eq!(4, stacks[2].len());
        assert_eq!('D', stacks[2][3]);
        assert_eq!('N', stacks[2][2]);
        assert_eq!('Z', stacks[2][1]);
        assert_eq!('P', stacks[2][0]);

    }
}