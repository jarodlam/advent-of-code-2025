use crate::solution::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    type Output = u32;

    fn part1(_input: &str) -> Self::Output {
        0
    }

    fn part2(_input: &str) -> Self::Output {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(Day01::part1(TEST_INPUT), 3);
    }

    #[test]
    fn test_part2() {}
}
