use aoc2022::util::read_line;
use std::{ collections::HashSet, slice::Chunks };

// fn share_char(parts: Chunks<char>) -> char {
//     let part1 = parts.next().unwrap();
//     let part2 = parts.next().unwrap();

//     let set1: HashSet<char> = HashSet::from_iter(part1);
//     let set2: HashSet<char> = HashSet::from_iter(part2);
//     let intersect = set1.intersection(&set2);
//     let char = intersect.into_iter().next().unwrap_or(&'1');
//     return *char;
// }

pub fn intersection(chars: Vec<Vec<char>>) -> Vec<char> {
    let mut intersect_result: Vec<char> = chars[0].clone();

    for temp_vec in chars {
        let unique_a: HashSet<char> = temp_vec.into_iter().collect();
        intersect_result = unique_a
            .intersection(&intersect_result.into_iter().collect())
            .map(|i| *i)
            .collect::<Vec<_>>();
    }
    intersect_result
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
    let parts = line
        .chunks(middle)
        .map(|c| c.to_vec())
        .collect::<Vec<Vec<char>>>();

    let share_char = intersection(parts)[0];
    let prio = prioritize(share_char);
    println!("{} -> Rucksack ({})", prio, share_char);
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