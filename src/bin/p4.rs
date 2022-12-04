use aoc2022::util::read_line;
use std::{ collections::HashSet };

fn string_to_range(range: String) -> HashSet<u32> {
    let (start_str, end_str) = range.split_once("-").unwrap();
    let start: u32 = start_str.parse().unwrap();
    let end: u32 = end_str.parse().unwrap();

    let mut v: Vec<u32> = Vec::with_capacity(100);
    v.extend(start..=end);
    let a_set: HashSet<_> = v.iter().copied().collect();
    a_set
}

fn find_interseting_pairs_part1(lines: &String) -> usize {
    let result = lines
        .lines()
        .map(|x| {
            let (section1, section2) = x.split_once(",").unwrap();
            let section1 = string_to_range(section1.to_string());
            let section2 = string_to_range(section2.to_string());

            let section1_contains_2 = section1.iter().all(|item| section2.contains(item));
            let section2_contains_1 = section2.iter().all(|item| section1.contains(item));
            section1_contains_2 || section2_contains_1
        })
        .filter(|x| { *x })
        .count();

    return result;
}

fn find_overlapping_pairs_part2(lines: &String) -> usize {
    let result = lines
        .lines()
        .map(|x| {
            let (section1, section2) = x.split_once(",").unwrap();

            let section1 = string_to_range(section1.to_string());
            let section2 = string_to_range(section2.to_string());
            let overlapping = section1.intersection(&section2);

            overlapping.count() > 0
        })
        .filter(|x| { *x })
        .count();

    return result;
}

fn main() {
    let lines = read_line("./input/p4.txt").unwrap();

    let result = find_interseting_pairs_part1(&lines);
    println!("Result Part 1: {}", result);

    let result = find_overlapping_pairs_part2(&lines);
    println!("Result Part 2: {}", result);
}