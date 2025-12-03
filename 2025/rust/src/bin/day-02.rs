use std::fs::read_to_string;

fn main() {
    println!("Day 2!");
    let input = read_to_string("../../inputs/day-02.txt").expect("error");
    let parsed = parse(&input);
    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .filter_map(|range_str| {
            let mut parts = range_str.trim().split('-');
            let start = parts.next()?.parse::<u64>().ok()?;
            let end = parts.next()?.parse::<u64>().ok()?;
            Some((start, end))
        })
        .collect()
}
fn part1(ids: &Vec<(u64, u64)>) -> u64 {
    let mut count: u64 = 0;
    for id in ids {
        let (num1, num2) = id;
        for i in *num1..=*num2 {
            let num = i.to_string();
            let half = num.len() / 2;
            if num[..half] == num[half..] {
                count += i;
            }
        }
    }
    return count;
}

fn part2(ids: &Vec<(u64, u64)>) -> u64 {
    let mut count: u64 = 0;
    for id in ids {
        let (num1, num2) = id;
        for i in *num1..=*num2 {
            let num = i.to_string();
            if is_valid(&num) {
                count += i;
            }
        }
    }
    count
}

fn is_valid(num: &str) -> bool {
    let n = num.len();

    for d in 1..=n / 2 {
        let slice = &num[..d];

        if slice.repeat(n / d) == num {
            return true;
        }
    }
    return false;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_01_test() {
        let parsed = parse(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124",
        );
        let result = part1(&parsed);
        assert_eq!(result, 1227775554)
    }
    #[test]
    fn part_02_test() {
        let parsed = parse(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124",
        );
        let result = part2(&parsed);
        assert_eq!(result, 4174379265);
    }
}
