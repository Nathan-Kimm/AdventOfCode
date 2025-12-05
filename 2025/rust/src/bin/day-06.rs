use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../../inputs/day-06.txt");
    println!("Day 6!");
}

fn part1(input: &str) -> i32 {
    1
}

#[cfg(test)]
mod test {
    use super::*;

    fn part_1_test() {
        let result = part1("");
        assert_eq!(result, 3);
    }
}
