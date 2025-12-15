use crate::solution::Solution;

pub struct Day04 {}

fn get_window<T>(grid: &[T], i: usize, j: usize) -> Vec<bool>
where
    T: std::ops::Deref<Target = [bool]>,
{
    let width = grid.len();
    let height = grid[0].len();
    let mut window = Vec::<bool>::new();
    if i > 0 && j > 0 {
        window.push(grid[i - 1][j - 1]);
    }
    if j > 0 {
        window.push(grid[i][j - 1]);
    }
    if i < width - 1 && j > 0 {
        window.push(grid[i + 1][j - 1]);
    }
    if i < width - 1 {
        window.push(grid[i + 1][j]);
    }
    if i < width - 1 && j < height - 1 {
        window.push(grid[i + 1][j + 1]);
    }
    if j < height - 1 {
        window.push(grid[i][j + 1]);
    }
    if i > 0 && j < height - 1 {
        window.push(grid[i - 1][j + 1]);
    }
    if i > 0 {
        window.push(grid[i - 1][j]);
    }
    window
}

fn find_removable_rolls<T>(grid: &[T]) -> Vec<(usize, usize)>
where
    T: std::ops::Deref<Target = [bool]>,
{
    let mut to_remove = Vec::<(usize, usize)>::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if !cell {
                continue;
            }
            let window = get_window(grid, i, j);
            let count = window.iter().filter(|x| **x).count();
            if count < 4 {
                to_remove.push((i, j));
            }
        }
    }
    to_remove
}

fn get_grid(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.bytes().map(|cell| cell == b'@').collect())
        .collect()
}

impl Solution for Day04 {
    type Output = u64;

    fn part1(input: &str) -> Self::Output {
        let grid = get_grid(input);
        find_removable_rolls(&grid).len() as u64
    }

    fn part2(input: &str) -> Self::Output {
        let mut out = 0;
        let mut grid = get_grid(input);
        loop {
            let removable_rolls = find_removable_rolls(&grid);
            if removable_rolls.is_empty() {
                break;
            }
            for (i, j) in removable_rolls {
                out += 1;
                grid[i][j] = false;
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(Day04::part1(TEST_INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day04::part2(TEST_INPUT), 43);
    }
}
