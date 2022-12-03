use std::{io::BufRead};

pub fn day1_part1(reader: &mut dyn BufRead) -> u32 {
    let mut elf_calories: Vec<u32> = Vec::new();
    let mut tmp: u32 = 0;

    for line in reader.lines() {

        //  Accumulate calories until an empty line is found
        match line {
            Ok(line_string) => {
                let line_trimmed = line_string.trim();
                if line_trimmed.is_empty() {
                    elf_calories.push(tmp);
                    tmp = 0;
                } else {
                    tmp += line_trimmed.parse::<u32>().unwrap();
                }
            },
            Err(_e) => eprintln!("erreh"),
        }
        if tmp != 0 {
            elf_calories.push(tmp);
        }
    }

    //  Find the largest calorie count
    let max_calories = elf_calories.iter().max().unwrap();

    return *max_calories;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part1_one_elf() {
        let input =
            "1000";
        let mut buf = input.as_bytes();
        let result = day1_part1(&mut buf);
        assert_eq!(1000, result);
    }

    #[test]
    fn day1_part1_one_elf_two_lines() {
        let input =
            "1000
            1000";
        let mut buf = input.as_bytes();
        let result = day1_part1(&mut buf);
        assert_eq!(2000, result);
    }

    #[test]
    fn day1_part1_two_elves() {
        let input =
            "1000
            1000
            
            1000
            2000";
        let mut buf = input.as_bytes();
        let result = day1_part1(&mut buf);
        assert_eq!(3000, result);
    }
}
