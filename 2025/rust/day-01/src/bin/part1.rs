use std::fs::read_to_string;

fn main() {
    println!("Part 1!");
    let input = read_to_string("input.txt").expect("error");
    println!("Output: {}", part1(&input));

    println!("Part 2!");
    println!("Output: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut dial = 50;
    let mut count = 0;
    for line in input.lines() {
        let (dir, num) = line.split_at(1);
        let num: i32 = num.parse().expect("unable to parse");
        if dir == "L" {
            dial = (dial - num).rem_euclid(100);
        } else {
            dial = (dial + num).rem_euclid(100);
        }
        if dial == 0 {
            count += 1;
        }
    }
    return count;
}

fn part2(input: &str) -> i32 {
    let mut dial = 50;
    let mut count = 0;
    for line in input.lines() {
        let (dir, num) = line.split_at(1);
        let num: i32 = num.parse().expect("unable to parse");
        let sign = if dir == "L" { -1 } else { 1 };
        for _ in 0..num {
            dial = (dial + sign) % 100;
            if dial == 0 {
                count += 1;
            }
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_01_test() {
        let result = part1(
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        );
        assert_eq!(result, 3);
    }

    #[test]
    fn part_02_test() {
        let result = part2(
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        );
        assert_eq!(result, 6);
    }
}
