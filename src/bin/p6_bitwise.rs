use std::collections::HashSet;
use aoc2022::util::read_line;

fn main() {
    let lines = read_line("./input/p6.txt").unwrap();

    let result = find_start_of_packet(&lines, 4);
    println!("Part 1 {}", result);

    let result = find_start_of_packet(&lines, 14);
    println!("Part 2 {}", result);
}

fn find_start_of_packet(s: &str, size: usize) -> usize {
    s
        .as_bytes()
        .windows(size)
        .position(|set| {
            let mut data: u32 = 0;
            for &c in set {
                let prev = data;
                data |= 1 << (c - b'a');
                if prev == data {
                    return false;
                }
            }
            return true;
        })
        .unwrap() + size
}