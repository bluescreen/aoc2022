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

fn repackage_part1(stacks: &mut Vec<Stack>, lines: &String) -> Vec<char> {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    for line in lines.lines() {
        let caps = re.captures(line).unwrap();
        let amount = caps[1].parse::<usize>().unwrap();
        let from = caps[2].parse::<usize>().unwrap();
        let to = caps[3].parse::<usize>().unwrap();

        move_cratemover_9000(stacks, amount, from, to);
    }
    let v: Vec<char> = stacks
        .iter()
        .map(|s| { s.last() })
        .collect();
    v
}

fn repackage_part2(stacks: &mut Vec<Stack>, lines: String) -> Vec<char> {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    for line in lines.lines() {
        let caps = re.captures(line).unwrap();
        let amount = caps[1].parse::<usize>().unwrap();
        let from = caps[2].parse::<usize>().unwrap();
        let to = caps[3].parse::<usize>().unwrap();

        move_cratemover_9001(stacks, amount, from, to);
    }
    let v: Vec<char> = stacks
        .iter()
        .map(|s| { s.last() })
        .collect();
    v
}

fn build_stacks_from_string(lines: String) -> Vec<Stack> {
    let mut v: Vec<Stack> = vec![];
    let mut num_columns = 0;

    for line in lines.lines().rev().skip(1) {
        let mut index = 1;
        if num_columns == 0 {
            num_columns = line
                .chars()
                .filter(|x| x.is_alphabetic())
                .count();
        }

        let bytes = line.as_bytes();
        for stack_index in 0..num_columns {
            if stack_index >= v.len() {
                v.push(Stack::new());
            }
            if (bytes[index] as char) != ' ' {
                v[stack_index].push(bytes[index] as char);
            }
            index += 4;
        }
    }

    v
}

fn main() {
    let lines = read_line("./input/p5.txt").unwrap();
    let mut splits = lines.split("\n\n").into_iter();
    let stack_strings = splits.next().unwrap().to_string();
    let instructions = splits.next().unwrap().to_string();

    let mut stacks = build_stacks_from_string(stack_strings.to_string());
    let result = repackage_part1(&mut stacks, &instructions);
    println!("Result Part 1: {:?}", vec_to_string(result));

    let mut stacks = build_stacks_from_string(stack_strings.to_string());
    let result = repackage_part2(&mut stacks, instructions);
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
    fn new() -> Self {
        Stack { stack: vec![] }
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