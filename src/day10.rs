use std::io::BufRead;

pub enum Mnemonic {
    Noop,
    Addx,
}

pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub cycle_count: usize,
    pub arg: i32,
}

pub struct FakeComputer {
    pub cycle: i32,
    pub ip: usize,
    pub ic: usize,
    pub x: i32,
    pub program: Vec<Instruction>,
}

impl FakeComputer {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self { cycle: 1, ip: 0, ic: 0, x: 1, program }
    }

    pub fn run(&mut self, count: i32) {
        //println!("running for {count} cycles");
        let end_cycle = self.cycle + count;

        while self.cycle < end_cycle {
            //println!("doing cycle {0}", self.cycle);
            self.ic += 1;

            if self.ic == self.program[self.ip].cycle_count {
                //  This instruction is complete, apply it
                match self.program[self.ip].mnemonic {
                    Mnemonic::Noop => {
                        //  do nothing
                    },
                    Mnemonic::Addx => {
                        //  add the argument to register x
                        self.x += self.program[self.ip].arg;
                    },
                }

                self.ip += 1;
                self.ic = 0;
                //println!("next instruction {0}", self.ip);
            }

            self.cycle += 1;
        }
    }

    pub fn get_signal_strength(&self) -> i32 {
        self.cycle * self.x
    }
}

/// Read input movements into a vector
pub fn read_program(reader: &mut dyn BufRead) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let trimmed_line = line.trim();

        let tokens = trimmed_line.split(" ").collect::<Vec<&str>>();
        
        match tokens[0] {
            "noop" => {
                instructions.push(Instruction { mnemonic: Mnemonic::Noop, cycle_count: 1, arg: 0 });
            },
            "addx" => {
                let arg = tokens[1].parse::<i32>().unwrap();
                instructions.push(Instruction { mnemonic: Mnemonic::Addx, cycle_count: 2, arg });
            },
            _ => panic!("invalid instruction"),
        }
    }

    return instructions;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run1() {
        let input = "addx 15";
        let mut buf = input.as_bytes();
        let program = read_program(&mut buf);
        assert_eq!(1, program.len());

        let mut cpu = FakeComputer::new(program);

        cpu.run(1);
        assert_eq!(1, cpu.x);

        cpu.run(1);
        assert_eq!(16, cpu.x);
    }

    #[test]
    fn run2() {
        let input = "addx 15
            addx -11";
        let mut buf = input.as_bytes();
        let program = read_program(&mut buf);
        assert_eq!(2, program.len());

        let mut cpu = FakeComputer::new(program);

        cpu.run(2);
        assert_eq!(16, cpu.x);

        cpu.run(2);
        assert_eq!(5, cpu.x);
    }

    #[test]
    fn run3() {
        let input = "addx 15
            noop
            addx -11";
        let mut buf = input.as_bytes();
        let program = read_program(&mut buf);
        assert_eq!(3, program.len());

        let mut cpu = FakeComputer::new(program);

        cpu.run(2);
        assert_eq!(16, cpu.x);

        cpu.run(2);
        assert_eq!(16, cpu.x);

        cpu.run(1);
        assert_eq!(5, cpu.x);
    }
}