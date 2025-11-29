use std::fs::File;
use std::io::Read;

fn main() {
    let mut file;
    file = File::open("../input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    part_one(&input);
}

fn part_one(input: &String) {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left_list.push(items.next().unwrap().parse::<i32>().unwrap());
        right_list.push(items.next().unwrap().parse::<i32>().unwrap());
    }
}
