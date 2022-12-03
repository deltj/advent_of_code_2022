use std::io::BufRead;

pub fn read_elf_calories(reader: &mut dyn BufRead) -> Vec<u32> {
    let mut elf_calorie_vector: Vec<u32> = Vec::new();
    let mut calorie_accumulator: u32 = 0;

    for line_result in reader.lines() {

        let line = line_result.unwrap();
        let line_trimmed = line.trim();

        //  Accumulate calories until an empty line is found
        if line_trimmed.is_empty() {
            elf_calorie_vector.push(calorie_accumulator);
            calorie_accumulator = 0;
        } else {
            calorie_accumulator += line_trimmed.parse::<u32>().unwrap();
        }
    }

    if calorie_accumulator != 0 {
        elf_calorie_vector.push(calorie_accumulator);
    }

    let l = elf_calorie_vector.len();
    println!("vector len: {l}");
    return elf_calorie_vector;
}

pub fn max_calories(elf_calorie_vector: &Vec<u32>) -> u32 {

    //  Find the largest calorie count
    let max_calories = elf_calorie_vector.iter().max().unwrap();

    return *max_calories;
}

pub fn top_three_calories(elf_calorie_vector: &mut Vec<u32>) -> (u32, u32, u32) {

    //  Sort the calorie vector, descending
    elf_calorie_vector.sort_unstable();
    elf_calorie_vector.reverse();

    let elf1 = if elf_calorie_vector.len() >= 1 { elf_calorie_vector[0] } else { 0 };
    let elf2 = if elf_calorie_vector.len() >= 2 { elf_calorie_vector[1] } else { 0 };
    let elf3 = if elf_calorie_vector.len() >= 3 { elf_calorie_vector[2] } else { 0 };

    return (elf1, elf2, elf3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_elf_one_line() {
        let input =
            "1000";
        let mut buf = input.as_bytes();
        let elf_calorie_vector = read_elf_calories(&mut buf);

        assert_eq!(1, elf_calorie_vector.len());
        assert_eq!(1000, elf_calorie_vector[0]);

        let max = max_calories(&elf_calorie_vector);

        assert_eq!(1000, max);
    }

    #[test]
    fn one_elf_two_lines() {
        let input =
            "1000
            1000";
        let mut buf = input.as_bytes();
        let elf_calorie_vector = read_elf_calories(&mut buf);

        assert_eq!(1, elf_calorie_vector.len());
        assert_eq!(2000, elf_calorie_vector[0]);

        let max = max_calories(&elf_calorie_vector);

        assert_eq!(2000, max);
    }

    #[test]
    fn two_elves() {
        let input =
            "1000
            1000
            
            1000
            2000";
        let mut buf = input.as_bytes();
        let elf_calorie_vector = read_elf_calories(&mut buf);

        assert_eq!(2, elf_calorie_vector.len());
        assert_eq!(2000, elf_calorie_vector[0]);
        assert_eq!(3000, elf_calorie_vector[1]);

        let max = max_calories(&elf_calorie_vector);

        assert_eq!(3000, max);
    }

    #[test]
    fn three_elves() {
        let input =
            "1000
            
            2000
            
            3000";
        let mut buf = input.as_bytes();
        let mut elf_calorie_vector = read_elf_calories(&mut buf);

        assert_eq!(3, elf_calorie_vector.len());
        assert_eq!(1000, elf_calorie_vector[0]);
        assert_eq!(2000, elf_calorie_vector[1]);
        assert_eq!(3000, elf_calorie_vector[2]);

        let max = max_calories(&elf_calorie_vector);

        assert_eq!(3000, max);

        let top = top_three_calories(&mut elf_calorie_vector);
        assert_eq!(3000, top.0);
        assert_eq!(2000, top.1);
        assert_eq!(1000, top.2);
    }
}