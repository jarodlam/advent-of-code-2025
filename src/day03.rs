use crate::solution::Solution;

pub struct Day03 {}

/// Returns the index of the maximum value in this iterator.
/// If there are multiple maximum values, return the earliest index.
fn earliest_max<I, V>(iterable: I) -> (usize, V)
where
    I: IntoIterator<Item = V>,
    V: PartialOrd,
{
    let mut iter = iterable.into_iter().enumerate();
    let (mut max_idx, mut max_val) = iter.next().unwrap();
    for (idx, val) in iter {
        if val > max_val {
            max_idx = idx;
            max_val = val;
        }
    }
    (max_idx, max_val)
}

fn largest_joltage(input: &str, num_batteries: usize) -> u64 {
    let mut out: u64 = 0;
    for line in input.lines() {
        let batteries = line.as_bytes();
        let mut jolts_str = Vec::<u8>::new();
        let mut left: usize = 0;

        for i in (0..num_batteries).rev() {
            let (idx, val) = earliest_max(&batteries[left..batteries.len() - i]);
            left += idx + 1;
            jolts_str.push(*val);
        }

        let jolts: u64 = std::str::from_utf8(jolts_str.as_slice())
            .unwrap()
            .parse()
            .unwrap();
        out += jolts;
    }
    out
}

impl Solution for Day03 {
    type Output = u64;

    fn part1(input: &str) -> Self::Output {
        largest_joltage(input, 2)
    }

    fn part2(input: &str) -> Self::Output {
        largest_joltage(input, 12)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(Day03::part1(TEST_INPUT), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day03::part2(TEST_INPUT), 3121910778619);
    }
}
