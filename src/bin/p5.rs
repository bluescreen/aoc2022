use aoc2022::util::read_line;
use regex::Regex;

fn move_cratemover_9000(stacks: &mut Vec<Stack>, amount: usize, from: usize, to: usize) {
    for _ in 0..amount {
        let item = stacks[from - 1].pop();
        if let Some(item) = item {
            stacks[to - 1].push(item);
        }
    }
}

fn move_cratemover_9001(stacks: &mut Vec<Stack>, amount: usize, from: usize, to: usize) {
    let mut temp_vec = vec![];
    for _ in 0..amount {
        let item = stacks[from - 1].pop();
        if let Some(item) = item {
            temp_vec.push(item);
        }
    }
    for item in temp_vec.iter().rev() {
        stacks[to - 1].push(*item);
    }
}

fn repackage_part1(lines: &String) -> Vec<char> {
    let mut stacks = build_stacks();

    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    for line in lines.lines() {
        let caps = re.captures(line).unwrap();
        let amount = caps[1].parse::<usize>().unwrap();
        let from = caps[2].parse::<usize>().unwrap();
        let to = caps[3].parse::<usize>().unwrap();

        move_cratemover_9000(&mut stacks, amount, from, to);
    }
    let v: Vec<char> = stacks
        .iter()
        .map(|s| { s.last() })
        .collect();
    v
}

fn repackage_part2(lines: String) -> Vec<char> {
    let mut stacks = build_stacks();

    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    for line in lines.lines() {
        let caps = re.captures(line).unwrap();
        let amount = caps[1].parse::<usize>().unwrap();
        let from = caps[2].parse::<usize>().unwrap();
        let to = caps[3].parse::<usize>().unwrap();

        move_cratemover_9001(&mut stacks, amount, from, to);
    }
    let v: Vec<char> = stacks
        .iter()
        .map(|s| { s.last() })
        .collect();
    v
}

fn build_stacks() -> Vec<Stack> {
    /*
        [H]         [S]         [D]
    [S] [C]         [C]     [Q] [L]
    [C] [R] [Z]     [R]     [H] [Z]
    [G] [N] [H] [S] [B]     [R] [F]
[D] [T] [Q] [F] [Q] [Z]     [Z] [N]
[Z] [W] [F] [N] [F] [W] [J] [V] [G]
[T] [R] [B] [C] [L] [P] [F] [L] [H]
[H] [Q] [P] [L] [G] [V] [Z] [D] [B]
 1   2   3   4   5   6   7   8   9 
 */
    let stacks =
        "HTZD
    QRWTGCS
    PBFQNRCH
    LCNFHZ
    GLFQS
    VPWZBRCS
    ZFJ
    DLVZRHQ
    BHGNFZLD";

    let stacks = stacks
        .lines()
        .map(|x| { Stack::from_vec(x.chars().collect()) })
        .collect();
    stacks
}

fn main() {
    let lines = read_line("./input/p5.txt").unwrap();

    let result = repackage_part1(&lines);
    println!("Result Part 1: {:?}", vec_to_string(result));

    let result = repackage_part2(lines);
    println!("Result Part 2: {:?}", vec_to_string(result));
}

fn vec_to_string(vec: Vec<char>) -> String {
    vec.iter()
        .map(|x| x.to_string())
        .collect()
}

#[derive(Debug)]
struct Stack {
    stack: Vec<char>,
}

impl Stack {
    fn from_vec(vec: Vec<char>) -> Self {
        Stack { stack: vec }
    }

    fn last(&self) -> char {
        let c = self.stack.last().unwrap();
        *c
    }

    fn pop(&mut self) -> Option<char> {
        self.stack.pop()
    }

    fn push(&mut self, item: char) {
        self.stack.push(item)
    }
}