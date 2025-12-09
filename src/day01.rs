use crate::solution::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    type Output = u32;

    fn part1(input: &str) -> Self::Output {
        let mut out: i32 = 0;
        let mut pos: i32 = 50;
        for line in input.lines() {
            let (dir_str, dist_str) = line.split_at(1);
            let dir: i32 = if dir_str == "L" { -1 } else { 1 };
            let dist: i32 = dist_str.parse().unwrap();

            pos = (pos + dir * dist).rem_euclid(100);

            if pos == 0 {
                out += 1;
            }
        }
        out as u32
    }

    fn part2(input: &str) -> Self::Output {
        let mut out: i32 = 0;
        let mut pos: i32 = 50;
        for line in input.lines() {
            let (dir_str, dist_str) = line.split_at(1);
            let dir: i32 = if dir_str == "L" { -1 } else { 1 };
            let dist: i32 = dist_str.parse().unwrap();

            if dir == 1 {
                out += (pos + dist) / 100;
            } else {
                out += ((100 - pos) % 100 + dist) / 100;
            }
            pos = (pos + dir * dist).rem_euclid(100);
        }
        out as u32
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
    fn test_part2() {
        assert_eq!(Day01::part2(TEST_INPUT), 6);
    }
}
