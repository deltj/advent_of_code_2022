use std::io::BufRead;

/// Read lines from the input buffer and store them in a vector
pub fn read_rucksack_vector(reader: &mut dyn BufRead) -> Vec<String> {
    let mut rucksack_vector: Vec<String> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let trimmed_line = line.trim();
        rucksack_vector.push(trimmed_line.to_string());
    }

    return rucksack_vector;
}

/// Find the set of characters common to the provided (2) strings
fn find_common_chars2(s1: &str, s2: &str) -> Vec<char> {
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

/// Find the set of characters common to the provided (3) strings
fn find_common_chars3(s1: &str, s2: &str, s3: &str) -> Vec<char> {
    let mut common_chars: Vec<char> = Vec::new();
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            if c1 == c2 {
                //  This character is common to s1 and s2
                for c3 in s3.chars() {
                    if c1 == c3 {
                        //  This character is common to s1 and s2 and s3
                        if !common_chars.contains(&c1) {
                            common_chars.push(c1);
                        }
                    }
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

/// Solve the day 3 part 1 problems
pub fn error_priority_sum(rucksack_vector: &Vec<String>) -> u32 {
    let mut sum = 0;
    for rucksack in rucksack_vector {
        let n = rucksack.len(); // n=length of input string

        if n % 2 != 0 {
            panic!("input strings must contain an even number of characters");
        }

        let c = n / 2;              // c=length of compartment strings
        let comp1 = &rucksack[0..c];
        let comp2 = &rucksack[c..n];
        let common_chars = find_common_chars2(&comp1, &comp2);
        for c in common_chars {
            sum += char_priority(&c);
        }
    }
    return sum;
}

/// Solve the day 3 part 2 problem
pub fn badge_priority_sum(rucksack_vector: &Vec<String>) -> u32 {
    let mut sum = 0;
    let n = rucksack_vector.len();  // n=number of rucksacks/elves

    if n % 3 != 0 {
        panic!("number of rucksacks/elves must be divisible by three");
    }

    let m = n / 3;  // m=number of groups
    println!("m={m}");

    for i in 0..m {
        // Find the badge for this group
        let common_chars = find_common_chars3(
            &rucksack_vector[i * 3],
            &rucksack_vector[(i * 3) + 1],
            &rucksack_vector[(i * 3) + 2]
        );
        
        if common_chars.len() != 1 {
            panic!("unexpected number of common characters in group {i}");
        }

        sum += char_priority(&common_chars[0]);
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_priority_sum1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let mut buf = input.as_bytes();
        let rucksack_vector = read_rucksack_vector(&mut buf);
        assert_eq!(1, rucksack_vector.len());

        let sum = error_priority_sum(&rucksack_vector);
        assert_eq!(16, sum);
    }

    #[test]
    fn error_priority_sum2() {
        let input = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let mut buf = input.as_bytes();
        let rucksack_vector = read_rucksack_vector(&mut buf);
        assert_eq!(1, rucksack_vector.len());

        let sum = error_priority_sum(&rucksack_vector);
        assert_eq!(38, sum);
    }

    #[test]
    fn error_priority_sum3() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let mut buf = input.as_bytes();
        let rucksack_vector = read_rucksack_vector(&mut buf);
        assert_eq!(2, rucksack_vector.len());

        let sum = error_priority_sum(&rucksack_vector);
        assert_eq!(54, sum);
    }

    #[test]
    fn badge_priority_sum1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg";
        let mut buf = input.as_bytes();
        let rucksack_vector = read_rucksack_vector(&mut buf);
        assert_eq!(3, rucksack_vector.len());

        let sum = badge_priority_sum(&rucksack_vector);
        assert_eq!(18, sum);
    }

    #[test]
    fn common_chars2_1() {
        let s1 = "abcd";
        let s2 = "defg";
        let cc = find_common_chars2(s1, s2);
        assert_eq!(1, cc.len());
        assert_eq!('d', cc[0]);
    }

    #[test]
    fn common_chars2_2() {
        let s1 = "aabcA";
        let s2 = "aabcB";
        let cc = find_common_chars2(s1, s2);
        assert_eq!(3, cc.len());
        assert_eq!('a', cc[0]);
        assert_eq!('b', cc[1]);
        assert_eq!('c', cc[2]);
    }

    #[test]
    fn common_chars3_1() {
        let s1 = "abcdefgA";
        let s2 = "hijklmnA";
        let s3 = "opqrstuA";
        let cc = find_common_chars3(s1, s2, s3);
        assert_eq!(1, cc.len());
        assert_eq!('A', cc[0]);
    }

    #[test]
    fn common_chars3_2() {
        let s1 = "abcdefgAAB";
        let s2 = "hijklmnAAB";
        let s3 = "opqrstuAAB";
        let cc = find_common_chars3(s1, s2, s3);
        assert_eq!(2, cc.len());
        assert_eq!('A', cc[0]);
        assert_eq!('B', cc[1]);
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
