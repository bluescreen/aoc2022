use std::collections::HashSet;

use aoc2022::util::read_line;
use itertools::Itertools;

fn main() {
    let lines = read_line("./input/p18.txt").unwrap();

    let result = solve(&lines);
    println!("Part 1: {}", result);
    // println!("Part 2: {}", max);
}

fn faces(x: i32, y: i32, z: i32) -> [(i32, i32, i32); 6] {
    return [
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ];
}

fn solve(lines: &str) -> i32 {
    let mut coords: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut count = 0;
    for line in lines.lines() {
        let (x, y, z) = line
            .split(",")
            .map(|x| { x.parse::<i32>().unwrap() })
            .collect_tuple()
            .unwrap();
        count += 6;
        for (cx, cy, cz) in faces(x, y, z) {
            if coords.contains(&(cx, cy, cz)) {
                count -= 2;
            }
        }
        coords.insert((x, y, z));
    }
    count
}