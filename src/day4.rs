use std::io::BufRead;

/// Read a string of the form "x-y" and return a tuple (x,y)
pub fn parse_sequence(s: &str) -> (u32,u32) {
    let bounds = s.split("-").collect::<Vec<&str>>();

    if bounds.len() != 2 {
        panic!("invalid set input: {s}");
    }

    let start = bounds[0].parse::<u32>().unwrap();
    let end   = bounds[1].parse::<u32>().unwrap();

    return (start, end);
}

#[derive(Debug,PartialEq)]
pub struct SequencePair {
    s1: (u32,u32),
    s2: (u32,u32),
}

/// Read pairs of sequences from the input buffer
pub fn read_sequence_pairs(reader: &mut dyn BufRead) -> Vec<SequencePair> {
    let mut seq_pair_vector: Vec<SequencePair> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let trimmed_line = line.trim();
        let sequences = trimmed_line.split(",").collect::<Vec<&str>>();
        
        if sequences.len() != 2 {
            panic!("invalid set pair input: {trimmed_line}");
        }

        let s1 = parse_sequence(sequences[0]);
        let s2 = parse_sequence(sequences[1]);
        let pair: SequencePair = SequencePair { s1, s2 };
        seq_pair_vector.push(pair);
    }

    return seq_pair_vector;
}

/// Test whether one of the specified sequences contains the other
fn sequence_contains(s1: (u32,u32), s2: (u32,u32)) -> bool {
    (s1.0 <= s2.0 && s1.1 >= s2.1) || (s2.0 <= s1.0 && s2.1 >= s1.1)
}

/// Test whether one of the specified sequences overlaps the other
fn sequence_overlaps(s1: (u32,u32), s2: (u32,u32)) -> bool {
    (s1.1 >= s2.0 && s1.0 <= s2.0) || (s2.1 >= s1.0 && s2.0 <= s1.0)
}

/// Count the number of sequence pairs with containment
/// (where one sequence in the pair contains the other)
pub fn count_contained_pairs(seq_pair_vector: &Vec<SequencePair>) -> u32 {
    let mut count = 0;

    for pair in seq_pair_vector {
        if sequence_contains(pair.s1, pair.s2) {
            count += 1;
        }
    }

    return count;
}

/// Count the number of sequence pairs with overlap
/// (where one sequence in the pair overlaps the other)
pub fn count_overlapped_pairs(seq_pair_vector: &Vec<SequencePair>) -> u32 {
    let mut count = 0;

    for pair in seq_pair_vector {
        if sequence_overlaps(pair.s1, pair.s2) {
            count += 1;
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sequence_test1() {
        let input = "2-4";
        let seq = parse_sequence(input);
        assert_eq!((2,4), seq);
    }

    #[test]
    fn parse_sequence_test2() {
        let input = "1000-2000";
        let seq = parse_sequence(input);
        assert_eq!((1000,2000), seq);
    }

    #[test]
    fn read_sequence_pairs_test1() {
        let input = "2-4,6-8";
        let mut buf = input.as_bytes();
        let sp = read_sequence_pairs(&mut buf);
        assert_eq!(1, sp.len());
        assert_eq!(SequencePair {s1: (2,4), s2: (6,8)}, sp[0]);
    }

    #[test]
    fn read_sequence_pairs_test2() {
        let input = "2-4,6-8
            2-3,4-5";
        let mut buf = input.as_bytes();
        let sp = read_sequence_pairs(&mut buf);
        assert_eq!(2, sp.len());
        assert_eq!(SequencePair {s1: (2,4), s2: (6,8)}, sp[0]);
        assert_eq!(SequencePair {s1: (2,3), s2: (4,5)}, sp[1]);
    }

    #[test]
    fn sequence_contains_test() {
        assert!( sequence_contains((2,8), (3,7)));
        assert!( sequence_contains((6,6), (4,6)));
        assert!(!sequence_contains((2,4), (6,8)));
    }

    #[test]
    fn sequence_overlaps_test() {
        assert!( sequence_overlaps((5,7), (7,9)));
        assert!( sequence_overlaps((2,8), (3,7)));
        assert!( sequence_overlaps((2,6), (4,8)));
        assert!(!sequence_overlaps((2,3), (4,5)));
    }

}