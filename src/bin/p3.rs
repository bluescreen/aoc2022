use aoc2022::util::read_line;
use std::collections::HashSet;

fn share_char(a: Vec<char>, b: Vec<char>) -> char {
    let set1: HashSet<char> = HashSet::from_iter(a);
    let set2: HashSet<char> = HashSet::from_iter(b);
    let intersect = set1.intersection(&set2);
    let char = intersect.into_iter().next().unwrap_or(&'1');
    return *char;
}

fn prioritize(c: char) -> usize {
    let number = c as i32;
    if number <= 90 {
        return (number - 38) as usize;
    } else {
        return (number - 96) as usize;
    }
}

fn prioritize_all(lines: String) -> Vec<usize> {
    let result: Vec<usize> = lines
        .lines()
        .map(|line| {
            let prio = prioritize_rucksack(line.chars().collect());
            prio
        })
        .collect();
    result
}

fn prioritize_rucksack(line: Vec<char>) -> usize {
    let middle: usize = (line.len() / 2) as usize;
    let mut parts = line.chunks(middle).into_iter();

    let part1 = parts.next().unwrap();
    let part2 = parts.next().unwrap();
    let share_char = share_char(part1.to_vec(), part2.to_vec());
    let prio = prioritize(share_char);
    println!("{} -> Rucksack {:?} - {:?} ({})", prio, part1, part2, share_char);
    prio
}

fn main() {
    let lines = read_line("./input/p3.txt").unwrap();
    let result = prioritize_all(lines);
    println!("Result {:?} = {}", result, result.iter().sum::<usize>());
}

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
// 16 (p), 38 (L), 42 (P), 22 (v), 20 (t)