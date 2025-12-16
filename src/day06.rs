use crate::solution::Solution;

pub struct Day06 {}

fn process<T>(numbers: &[T], operators: &[char]) -> u64
where
    T: std::ops::Deref<Target = [u64]>,
{
    numbers
        .iter()
        .zip(operators.iter())
        .map(|(ns, op)| {
            if *op == '+' {
                (*ns).iter().sum::<u64>()
            } else {
                (*ns).iter().product::<u64>()
            }
        })
        .sum()
}

impl Solution for Day06 {
    type Output = u64;

    fn part1(input: &str) -> Self::Output {
        let grid: Vec<Vec<&str>> = input
            .lines()
            .map(|line| line.split_whitespace().collect())
            .collect();
        let numbers: Vec<Vec<u64>> = (0..grid[0].len())
            .map(|col| {
                (0..grid.len() - 1)
                    .map(|row| grid[row][col].parse().unwrap())
                    .collect()
            })
            .collect();
        let operators: Vec<char> = grid[grid.len() - 1]
            .iter()
            .map(|cell| cell.chars().next().unwrap())
            .collect();
        process(&numbers, &operators)
    }

    fn part2(input: &str) -> Self::Output {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let numbers: Vec<Vec<u64>> = (0..grid[0].len())
            .map(|col| {
                (0..grid.len() - 1)
                    .filter(|&row| grid[row][col] != ' ')
                    .map(|row| grid[row][col])
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap_or(u64::MAX)
            })
            .collect::<Vec<u64>>()
            .split(|&x| x == u64::MAX)
            .map(|x| x.to_vec())
            .collect();
        let operators: Vec<char> = grid[grid.len() - 1]
            .iter()
            .filter(|&&cell| cell != ' ')
            .copied()
            .collect();
        process(&numbers, &operators)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        assert_eq!(Day06::part1(TEST_INPUT), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day06::part2(TEST_INPUT), 3263827);
    }
}
