use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../../inputs/day-03.txt").unwrap();
    println!("Day 3!");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let mut count = 0;

    for line in input.lines() {
        let mut max1 = 0;
        let mut index = 0;
        let mut max2 = 0;

        for i in 0..line.len() - 1 {
            let num: u32 = line.chars().nth(i).unwrap().to_digit(10).unwrap();
            if num > max1 {
                max1 = num;
                index = i + 1;
            }
        }

        for i in index as usize..line.len() {
            let num: u32 = line.chars().nth(i).unwrap().to_digit(10).unwrap();
            if num > max2 {
                max2 = num;
            }
        }
        count += max1 * 10 + max2;
    }
    return count;
}

fn part2(input: &str) -> u64 {
    let mut count = 0;

    for line in input.lines() {
        let mut res = String::new();
        let mut nums = 12;
        let mut index = 0;
        while nums != 0 {
            let mut max = 0;
            for i in index..=line.len() - nums {
                let num = line.chars().nth(i).unwrap().to_digit(10).unwrap();
                if num > max {
                    max = num;
                    index = i + 1;
                }
            }
            res += &max.to_string();
            nums -= 1;
        }
        let res: u64 = res.parse().unwrap();
        count += res;
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_01_test() {
        let result = part1(
            "987654321111111
811111111111119
234234234234278
818181911112111",
        );
        assert_eq!(result, 357);
    }

    #[test]
    fn part_02_test() {
        let result = part2(
            "987654321111111
811111111111119
234234234234278
818181911112111",
        );
        assert_eq!(result, 3121910778619);
    }
}
