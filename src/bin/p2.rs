use std::collections::HashMap;
use aoc2022::util::read_line;

// X Rock, Y for Paper, and Z for Scissors
// A Rock, B for Paper, and C for Scissors

fn score_rules_part1() -> HashMap<&'static str, u32> {
    let mut rules: HashMap<&str, u32> = HashMap::new();
    rules.insert("A X", 1 + 3);
    rules.insert("A Y", 2 + 6);
    rules.insert("A Z", 3 + 0);
    rules.insert("B X", 1 + 0);
    rules.insert("B Y", 2 + 3);
    rules.insert("B Z", 3 + 6);
    rules.insert("C X", 1 + 6);
    rules.insert("C Y", 2 + 0);
    rules.insert("C Z", 3 + 3);
    rules
}

fn score_rules_part2() -> HashMap<&'static str, u32> {
    let mut rules: HashMap<&str, u32> = HashMap::new();
    rules.insert("A X", 3 + 0);
    rules.insert("A Y", 1 + 3);
    rules.insert("A Z", 2 + 6);
    rules.insert("B X", 1 + 0);
    rules.insert("B Y", 2 + 3);
    rules.insert("B Z", 3 + 6);
    rules.insert("C X", 2 + 0);
    rules.insert("C Y", 3 + 3);
    rules.insert("C Z", 1 + 6);
    rules
}

fn score_games(lines: &String, hm: HashMap<&str, u32>) -> u32 {
    let scores: u32 = lines
        .split("\n")
        .map(|line| {
            return hm.get(line).unwrap_or(&0);
        })
        .sum();
    scores
}

fn main() {
    let lines = read_line("./input/p2.txt").unwrap();

    let scores = score_games(&lines, score_rules_part1());
    println!("Part 1 {:?}", scores);

    let scores = score_games(&lines, score_rules_part2());
    println!("Part 2 {:?}", scores);
}