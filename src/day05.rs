use crate::solution::Solution;

pub struct Day05 {}

fn parse_and_merge_ranges(input: &str) -> Vec<(u64, u64)> {
    let mut ranges: Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            let (left_str, right_str) = line.split_once("-").unwrap();
            (left_str.parse().unwrap(), right_str.parse().unwrap())
        })
        .collect();

    ranges.sort_by_key(|x| x.0);
    let mut merged_ranges = vec![ranges[0]];
    for r in &ranges[1..] {
        let prev_idx = merged_ranges.len() - 1;
        let prev = merged_ranges[prev_idx];
        let prev_right = prev.1 + 1;
        let curr_left = r.0;
        let curr_right = r.1 + 1;
        if (curr_left <= prev_right) && (prev_right < curr_right) {
            // Partially overlapping, merge
            merged_ranges[prev_idx].1 = r.1;
        } else if prev_right < curr_left {
            // Non-overlapping, push
            merged_ranges.push(*r);
        }
        // Fully overlapping, skip
    }
    merged_ranges
}

fn is_fresh(ranges: &[(u64, u64)], id: u64) -> bool {
    for range in ranges {
        if id >= range.0 && id <= range.1 {
            return true;
        }
    }
    false
}

impl Solution for Day05 {
    type Output = u64;

    fn part1(input: &str) -> Self::Output {
        let (input_fresh, input_ids) = input.split_once("\n\n").unwrap();
        let ranges = parse_and_merge_ranges(input_fresh);
        input_ids
            .lines()
            .filter(|x| is_fresh(&ranges, x.parse().unwrap()))
            .count()
            .try_into()
            .unwrap()
    }

    fn part2(input: &str) -> Self::Output {
        let (input_fresh, _) = input.split_once("\n\n").unwrap();
        let ranges = parse_and_merge_ranges(input_fresh);
        println!("{:?}", ranges);
        ranges.iter().map(|x| x.1 - x.0 + 1).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        assert_eq!(Day05::part1(TEST_INPUT), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day05::part2(TEST_INPUT), 14);
    }
}
