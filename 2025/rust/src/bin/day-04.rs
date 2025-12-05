use std::fs::read_to_string;

const DIRS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, 1),
    (-1, -1),
    (1, 1),
    (1, -1),
];

fn main() {
    let input = read_to_string("../../inputs/day-04.txt").unwrap();
    println!("Day 4!");
    let parsed = parse(&input);
    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

fn parse(input: &str) -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = Vec::new();
    for line in input.lines() {
        let mut chars: Vec<bool> = Vec::new();
        for c in line.chars() {
            if c == '@' {
                chars.push(true);
            } else {
                chars.push(false);
            }
        }
        grid.push(chars);
    }
    return grid;
}

fn part1(grid: &Vec<Vec<bool>>) -> u32 {
    let mut count = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == true {
                if num_surrounding(&grid, r as i32, c as i32) < 4 {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part2(grid: &Vec<Vec<bool>>) -> u32 {
    let mut grid_copy = grid.clone();
    let mut count = 0;
    let mut old_count = 0;
    while true {
        old_count = count;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid_copy[r][c] == true {
                    if num_surrounding(&grid_copy, r as i32, c as i32) < 4 {
                        grid_copy[r][c] = false;
                        count += 1;
                    }
                }
            }
        }
        if old_count == count {
            break;
        }
    }
    count
}

fn num_surrounding(grid: &Vec<Vec<bool>>, r: i32, c: i32) -> u32 {
    let mut count = 0;

    for (dr, dc) in DIRS.iter() {
        let nr = r + dr;
        let nc = c + dc;

        if nr >= 0 && nr < grid.len() as i32 && nc >= 0 && nc < grid[0].len() as i32 {
            let nr = nr as usize;
            let nc = nc as usize;

            let neighbor_value = grid[nr][nc];

            if neighbor_value == true {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let parsed = parse(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        );
        let result = part1(&parsed);
        assert_eq!(result, 13);
    }

    #[test]
    fn part_2_test() {
        let parsed = parse(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        );
        let result = part2(&parsed);
        assert_eq!(result, 43);
    }
}
