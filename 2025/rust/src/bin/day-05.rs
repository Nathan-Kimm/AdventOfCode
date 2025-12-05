use std::fs::read_to_string;

fn main() {
    println!("Day 5!");
    let input = read_to_string("../../inputs/day-05.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let mut ids: Vec<(u64, u64)> = Vec::new();
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    let mut count = 0;

    for range in ranges.lines() {
        let (start, end) = range.split_once('-').unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();

        ids.push((start, end));
    }

    for ingredient in ingredients.lines() {
        let ingredient: u64 = ingredient.parse().unwrap();
        for (start, end) in &ids {
            if ingredient >= *start && ingredient <= *end {
                count += 1;
                break;
            }
        }
    }

    count
}

fn part2(input: &str) -> u64 {
    let mut ids: Vec<(u64, u64)> = Vec::new();
    let mut count: u64 = 0;
    let (ranges, _) = input.split_once("\n\n").unwrap();
    for line in ranges.lines() {
        let (start, end) = line.split_once('-').unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        ids.push((start, end));
    }

    ids.sort();

    let mut previous: u64 = 0;
    for (start, end) in ids {
        if start > previous {
            count += end - start + 1;
            previous = end;
        } else if end > previous {
            count += end - previous;
            previous = end;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let result = part1(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        );
        assert_eq!(result, 3);
    }

    #[test]
    fn part_2_test() {
        let result = part2(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        );
        assert_eq!(result, 14);
    }
}
