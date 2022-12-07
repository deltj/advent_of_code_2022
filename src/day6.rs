
pub fn find_sop(s: &str) -> usize {
    let n: usize = s.len();

    for i in 0..n {
        //println!("i={i}");
        let mut dup = false;
        for j in 0..4 {
            //println!("j={j}");
            for k in (j+1)..4 {
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
            return i + 4;
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
        let sop = find_sop(input);
        assert_eq!(7, sop);
    }

    #[test]
    fn find_sop_test2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let sop = find_sop(input);
        assert_eq!(5, sop);
    }

    #[test]
    fn find_sop_test3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let sop = find_sop(input);
        assert_eq!(6, sop);
    }

    #[test]
    fn find_sop_test4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let sop = find_sop(input);
        assert_eq!(10, sop);
    }

    #[test]
    fn find_sop_test5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let sop = find_sop(input);
        assert_eq!(11, sop);
    }
}