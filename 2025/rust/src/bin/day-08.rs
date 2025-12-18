use std::fs::read_to_string;

struct Box(u64, u64, u64);

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let rep_a = self.find(a);
        let rep_b = self.find(b);

        if rep_a == rep_b {
            return false;
        }
        self.parent[rep_b] = rep_a;
        self.size[rep_a] += self.size[rep_b];
        true
    }

    fn component_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}

fn main() {
    let input = read_to_string("../../inputs/day-08.txt").unwrap();
    println!("Day 8!");
    let boxes = parse(&input);
    println!("Part 1: {}", part1(&boxes));
    println!("Part 2: {}", part2(&boxes));
}

fn parse(input: &str) -> Vec<Box> {
    let mut boxes: Vec<Box> = input
        .lines()
        .map(|line| {
            let nums: Vec<u64> = line
                .split(',')
                .map(|num| num.parse().expect("Invalid number"))
                .collect();
            Box(nums[0], nums[1], nums[2])
        })
        .collect();

    boxes
}

fn part1(input: &Vec<Box>) -> u64 {
    let mut uf = UnionFind::new(input.len());
    let mut edges: Vec<(u64, usize, usize)> = Vec::new();

    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            let d = distance(&input[i], &input[j]);
            edges.push((d, i, j));
        }
    }
    edges.sort_unstable_by_key(|e| e.0);
    let mut count = 0;
    for &(_, a, b) in edges.iter() {
        if count == 1000 {
            break;
        }
        uf.union(a, b);
        count += 1;
    }

    let mut sizes = Vec::new();
    for i in 0..input.len() {
        if uf.find(i) == i {
            sizes.push(uf.component_size(i) as u64);
        }
    }
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    let res = sizes[0] * sizes[1] * sizes[2];
    res
}

fn part2(input: &Vec<Box>) -> u64 {
    let mut uf = UnionFind::new(input.len());
    let mut edges: Vec<(u64, usize, usize)> = Vec::new();
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            let d = distance(&input[i], &input[j]);
            edges.push((d, i, j));
        }
    }

    edges.sort_unstable_by_key(|e| e.0);
    let mut count = input.len();
    for &(_, a, b) in edges.iter() {
        if uf.union(a, b) {
            count -= 1;
            if count == 1 {
                return input[a].0 * input[b].0;
            }
        }
    }
    1
}

fn distance(a: &Box, b: &Box) -> u64 {
    a.0.abs_diff(b.0).pow(2) + a.1.abs_diff(b.1).pow(2) + a.2.abs_diff(b.2).pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let parsed = parse(
            "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
        );
        let result = part1(&parsed);
        assert_eq!(result, 40);
    }

    #[test]
    fn part_2_test() {
        let parsed = parse(
            "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
        );
        let result = part2(&parsed);
        assert_eq!(result, 25272);
    }
}
