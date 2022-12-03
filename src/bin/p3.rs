use aoc2022::util::read_line;
use std::{ collections::HashSet };

pub fn find_intersection(chars: Vec<Vec<char>>) -> Vec<char> {
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

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
// 16 (p), 38 (L), 42 (P), 22 (v), 20 (t)
fn prioritize(c: char) -> usize {
    let number = c as i32;
    if number <= 90 {
        return (number - 38) as usize;
    } else {
        return (number - 96) as usize;
    }
}

fn prioritize_rucksack(line: Vec<char>) -> usize {
    let middle: usize = (line.len() / 2) as usize;
    let parts = line
        .chunks(middle)
        .map(|c| c.to_vec())
        .collect::<Vec<Vec<char>>>();

    let share_char = find_intersection(parts)[0];
    let prio = prioritize(share_char);
    prio
}

fn prioritize_part1(lines: String) -> Vec<usize> {
    let result: Vec<usize> = lines
        .lines()
        .map(|line| {
            let prio = prioritize_rucksack(line.chars().collect());
            prio
        })
        .collect();
    result
}

fn group_lines(lines: String, num: usize) -> Vec<Vec<String>> {
    let grouped: Vec<Vec<String>> = lines.lines().fold(Vec::new(), |mut acc, l| {
        if acc.last().is_some() && acc.last().unwrap().len() < num {
            acc.last_mut().unwrap().push(l.to_string());
        } else {
            acc.push(vec![l.to_string()]);
        }
        acc
    });
    grouped
}

fn prioritize_part2(lines: String) -> Vec<usize> {
    let grouped = group_lines(lines, 3);

    let result = grouped
        .iter()
        .map(|group| {
            let group = group
                .iter()
                .map(|x| x.chars().collect())
                .collect();
            let share_char = find_intersection(group)[0];
            let prio = prioritize(share_char);
            prio
        })
        .collect();
    result
}

fn main() {
    let lines = read_line("./input/p3.txt").unwrap();

    let result = prioritize_part1(lines);
    println!("Result Part 1 {}", result.iter().sum::<usize>());

    let lines = read_line("./input/p3.txt").unwrap();

    let result = prioritize_part2(lines);
    println!("Result Part 1 {}", result.iter().sum::<usize>());
}