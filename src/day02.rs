use crate::solution::Solution;

pub struct Day02 {}

fn is_repeated_twice(n: u64) -> bool {
    let n_str = format!("{}", n);
    let n_bytes = n_str.as_bytes();
    let len = n_str.len();

    // Odd
    if len % 2 == 1 {
        return false;
    }

    // Even
    let mut itr = n_bytes.chunks(len / 2);
    let first_chunk = itr.next().unwrap();
    if itr.all(|x| x == first_chunk) {
        return true;
    }

    false
}

fn is_repeated_n(n: u64) -> bool {
    let n_str = format!("{}", n);
    let n_bytes = n_str.as_bytes();
    let len = n_str.len();

    for chunk_size in (1..=(len / 2)).rev() {
        if len % chunk_size != 0 {
            continue;
        }

        let mut itr = n_bytes.chunks(chunk_size);
        let first_chunk = itr.next().unwrap();
        if itr.all(|x| x == first_chunk) {
            return true;
        }
    }

    false
}

impl Solution for Day02 {
    type Output = u64;

    fn part1(input: &str) -> Self::Output {
        let mut out: u64 = 0;
        for pair in input.trim().split(",") {
            let (start_str, end_str) = pair.split_once("-").unwrap();
            let start: u64 = start_str.parse().unwrap();
            let end: u64 = end_str.parse().unwrap();
            for i in start..=end {
                if is_repeated_twice(i) {
                    out += i;
                }
            }
        }
        out
    }

    fn part2(input: &str) -> Self::Output {
        let mut out: u64 = 0;
        for pair in input.trim().split(",") {
            let (start_str, end_str) = pair.split_once("-").unwrap();
            let start: u64 = start_str.parse().unwrap();
            let end: u64 = end_str.parse().unwrap();
            for i in start..=end {
                if is_repeated_n(i) {
                    out += i;
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(Day02::part1(TEST_INPUT), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day02::part2(TEST_INPUT), 4174379265);
    }
}
