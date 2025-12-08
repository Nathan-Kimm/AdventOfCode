use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../../inputs/day-07.txt").unwrap();
    println!("Day 7!");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut count = 0;
    let first_line = lines.next().unwrap().as_bytes();
    let start_pos = first_line.iter().position(|c| *c == b'S').unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        grid.push(line.chars().collect());
    }
    grid[0][start_pos] = '|';
    for r in 0..grid.len() - 1 {
        for c in 0..grid[0].len() {
            if grid[r][c] == '|' {
                if grid[r + 1][c] == '^' {
                    count += 1;
                    grid[r + 1][c + 1] = '|';
                    grid[r + 1][c - 1] = '|';
                } else {
                    grid[r + 1][c] = '|';
                }
            }
        }
    }

    count
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap().as_bytes();
    let start_pos = first_line.iter().position(|c| *c == b'S').unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        grid.push(line.chars().collect());
    }
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count: Vec<Vec<u64>> = vec![vec![0; cols]; rows];
    grid[0][start_pos] = '|';
    count[0][start_pos] = 1;
    for r in 0..rows - 1 {
        for c in 0..cols {
            if grid[r][c] == '|' {
                if grid[r + 1][c] == '^' {
                    grid[r + 1][c + 1] = '|';
                    grid[r + 1][c - 1] = '|';
                    count[r + 1][c + 1] += count[r][c];
                    count[r + 1][c - 1] += count[r][c];
                } else {
                    grid[r + 1][c] = '|';
                    count[r + 1][c] += count[r][c];
                }
            }
        }
    }
    let mut sum = 0;
    for n in 0..cols {
        sum += count[rows - 1][n];
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let result = part1(
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );
        assert_eq!(result, 21);
    }

    #[test]
    fn part_2_test() {
        let result = part2(
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );
        assert_eq!(result, 40);
    }
}
