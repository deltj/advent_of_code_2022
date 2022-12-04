use std::io::BufRead;

pub struct Rucksack {
    compartment1: String,
    compartment2: String,
}

/// Read lines from the input buffer and interpret them as described in AOC day 3,
/// storing the results in a vector of Rucksack structures
pub fn read_rucksack_vector(reader: &mut dyn BufRead) -> Vec<Rucksack> {
    let mut rucksack_vector: Vec<Rucksack> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let trimmed_line = line.trim();
        let n = trimmed_line.len(); // n=length of input string

        if n % 2 != 0 {
            panic!("input strings must contain an even number of characters");
        }

        let c = n / 2;              // c=length of compartment strings
        let comp1 = &trimmed_line[0..c];
        let comp2 = &trimmed_line[c..n];
        let rs: Rucksack = Rucksack {
            compartment1: comp1.to_string(),
            compartment2: comp2.to_string()
        };
        rucksack_vector.push(rs);
    }

    return rucksack_vector;
}

/// Find the set of characters common to the provided strings
fn find_common_chars(s1: &str, s2: &str) -> Vec<char> {
    let mut common_chars: Vec<char> = Vec::new();
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            if c1 == c2 {
                //  This character is common to both strings
                if !common_chars.contains(&c1) {
                    common_chars.push(c1);
                }
            }
        }
    }
    return common_chars;
}

/// Return the item priority for characters a-z and A-Z as described in the 
/// AOC '22 day 3 description
fn char_priority(c: &char) -> u32 {
    let ord = *c as u32;
    match ord {
        97..=122 => ord - 96,
        65..=90  => ord - 38,
        _        => 0,
    }
}

/// Solve the day 3 part 1 problem
pub fn error_priority_sum(rucksack_vector: &Vec<Rucksack>) -> u32 {
    let mut sum = 0;
    for rucksack in rucksack_vector {
        let common_chars = find_common_chars(&rucksack.compartment1, &rucksack.compartment2);
        for c in common_chars {
            sum += char_priority(&c);
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn read_rucksack_vector_bad_input() {
        let input = "abcde";
        let mut buf = input.as_bytes();
        let _rucksack_vector = read_rucksack_vector(&mut buf);
    }

    #[test]
    fn one_line1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let mut buf = input.as_bytes();
        let rucksack_vector = read_rucksack_vector(&mut buf);
        assert_eq!(1, rucksack_vector.len());
        assert_eq!("vJrwpWtwJgWr", rucksack_vector[0].compartment1);
        assert_eq!("hcsFMMfFFhFp", rucksack_vector[0].compartment2);

        let common_chars = find_common_chars(&rucksack_vector[0].compartment1, &rucksack_vector[0].compartment2);
        assert_eq!(1, common_chars.len());
        assert_eq!('p', common_chars[0]);

        let sum = error_priority_sum(&rucksack_vector);
        assert_eq!(16, sum);
    }

    #[test]
    fn one_line2() {
        let input = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let mut buf = input.as_bytes();
        let rucksack_vector = read_rucksack_vector(&mut buf);
        assert_eq!(1, rucksack_vector.len());
        assert_eq!("jqHRNqRjqzjGDLGL", rucksack_vector[0].compartment1);
        assert_eq!("rsFMfFZSrLrFZsSL", rucksack_vector[0].compartment2);

        let common_chars = find_common_chars(&rucksack_vector[0].compartment1, &rucksack_vector[0].compartment2);
        assert_eq!(1, common_chars.len());
        assert_eq!('L', common_chars[0]);

        let sum = error_priority_sum(&rucksack_vector);
        assert_eq!(38, sum);
    }

    #[test]
    fn two_lines() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let mut buf = input.as_bytes();
        let rucksack_vector = read_rucksack_vector(&mut buf);
        assert_eq!(2, rucksack_vector.len());
        assert_eq!("vJrwpWtwJgWr", rucksack_vector[0].compartment1);
        assert_eq!("hcsFMMfFFhFp", rucksack_vector[0].compartment2);
        assert_eq!("jqHRNqRjqzjGDLGL", rucksack_vector[1].compartment1);
        assert_eq!("rsFMfFZSrLrFZsSL", rucksack_vector[1].compartment2);

        let sum = error_priority_sum(&rucksack_vector);
        assert_eq!(54, sum);
    }

    #[test]
    fn common_chars1() {
        let s1 = "abcd";
        let s2 = "defg";
        let cc = find_common_chars(s1, s2);
        assert_eq!(1, cc.len());
        assert_eq!('d', cc[0]);
    }

    #[test]
    fn common_chars2() {
        let s1 = "aabcA";
        let s2 = "aabcB";
        let cc = find_common_chars(s1, s2);
        assert_eq!(3, cc.len());
        assert_eq!('a', cc[0]);
        assert_eq!('b', cc[1]);
        assert_eq!('c', cc[2]);
    }

    #[test]
    fn priority() {
        assert_eq!(1,  char_priority(&'a'));
        assert_eq!(26, char_priority(&'z'));
        assert_eq!(27, char_priority(&'A'));
        assert_eq!(38, char_priority(&'L'));
        assert_eq!(52, char_priority(&'Z'));
    }

}
