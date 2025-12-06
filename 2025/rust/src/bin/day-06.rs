use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../../inputs/day-06.txt").unwrap();
    println!("Day 6!");
    let parsed = parse(&input);
    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&input));
}

fn parse(input: &str) -> Vec<Vec<&str>> {
    let mut grid: Vec<Vec<&str>> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        let row: Vec<&str> = line.split_whitespace().collect();
        grid.push(row);
    }
    grid
}

fn part1(input: &Vec<Vec<&str>>) -> u64 {
    let mut sum: u64 = 0;
    for c in 0..input[0].len() {
        let mut count = 0;
        let operator = input[input.len() - 1][c];
        if operator == "*" {
            count += 1;
        }
        for r in 0..input.len() - 1 {
            let num: u64 = input[r][c].parse().unwrap();
            if operator == "+" {
                count += num;
            } else {
                count *= num;
            }
        }
        sum += count;
    }
    sum
}

fn part2(input: &str) -> u64 {
    let mut sum: u64 = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..cols {
        if grid[rows - 1][i] != ' ' {
            operators.push(grid[rows - 1][i]);
        }
    }

    let mut k = operators.len() - 1;
    let mut num = String::new();
    let mut count = if operators[k] == '+' { 0 } else { 1 };

    for col in (0..cols).rev() {
        num = String::new();
        for row in 0..rows - 1 {
            if grid[row][col] != ' ' {
                num.push(grid[row][col]);
            }
        }
        if num == "" {
            k -= 1;
            sum += count;
            count = if operators[k] == '+' { 0 } else { 1 };
            continue;
        } else {
            let num: u64 = num.parse().unwrap();
            if operators[k] == '+' {
                count += num;
            } else {
                count *= num;
            }
        }
    }

    sum += count;
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_test() {
        let parsed = parse(
            "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
        );
        assert_eq!(part1(&parsed), 4277556);
    }

    #[test]
    fn part_2_test() {
        let result = part2(
            "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
        );
        assert_eq!(result, 3263827);
    }
}
