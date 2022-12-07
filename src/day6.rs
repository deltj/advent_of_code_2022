
/// Find the end of the start of packet (or start of message) indicator
/// by returning the number of characters that need to be processed before
/// the sop/som is detected
pub fn find_sop(s: &str, d: usize) -> usize {
    let n: usize = s.len();

    for i in 0..n {
        //println!("i={i}");
        let mut dup = false;
        for j in 0..d {
            //println!("j={j}");
            for k in (j+1)..d {
                //println!("k={k}");
                let c1 = s.chars().nth(i + j).unwrap();
                let c2 = s.chars().nth(i + k).unwrap();
                //println!("comparing {c1} and {c2}");

                if c1 == c2 {
                    dup = true;
                }
            }
        }

        if !dup {
            return i + d;
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_sop_test1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(7, find_sop(input, 4));
        assert_eq!(19, find_sop(input, 14));
    }

    #[test]
    fn find_sop_test2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(5, find_sop(input, 4));
        assert_eq!(23, find_sop(input, 14));
    }

    #[test]
    fn find_sop_test3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(6, find_sop(input, 4));
        assert_eq!(23, find_sop(input, 14));
    }

    #[test]
    fn find_sop_test4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(10, find_sop(input, 4));
        assert_eq!(29, find_sop(input, 14));
    }

    #[test]
    fn find_sop_test5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(11, find_sop(input, 4));
        assert_eq!(26, find_sop(input, 14));
    }
}