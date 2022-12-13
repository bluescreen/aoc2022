use aoc2022::util::read_line;
use itertools::Itertools;
use sscanf::sscanf;

fn main() {
    let lines = read_line("./input/p11.txt").unwrap();

    let monkeys: Vec<Monkey> = lines.split("\n\n").map(build_monkey).collect();
    println!("Part 1: {}", solve(monkeys.clone(), 20, 3));
    println!("Part 2: {}", solve(monkeys, 10000, 1));
}

#[derive(Clone)]
enum Operation {
    Mul(i64),
    Add(i64),
    MulSelf,
    AddSelf,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test_divisor: i64,
    monkey_for_true: usize,
    monkey_for_false: usize,
    counter: usize,
}

fn solve(mut monkeys: Vec<Monkey>, rounds: usize, divisor: i64) -> usize {
    for _ in 0..rounds {
        step(&mut monkeys, divisor);
    }
    monkeys
        .iter()
        .map(|monkey| monkey.counter)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn step(monkeys: &mut Vec<Monkey>, divisor: i64) {
    let lcm = check_divisors(monkeys);
    for i in 0..monkeys.len() {
        let new_items = monkeys[i].items
            .iter()
            .map(
                |item|
                    ((match monkeys[i].operation {
                        Operation::Mul(rhs) => item * rhs,
                        Operation::Add(rhs) => item + rhs,
                        Operation::MulSelf => item * item,
                        Operation::AddSelf => item + item,
                    }) /
                        divisor) %
                    lcm
            )
            .collect::<Vec<_>>();
        monkeys[i].items.clear();
        monkeys[i].counter += new_items.len();
        next_monkey(new_items, monkeys, i);
    }
}

fn next_monkey(new_items: Vec<i64>, monkeys: &mut Vec<Monkey>, i: usize) {
    for &new_item in new_items.iter() {
        if new_item % monkeys[i].test_divisor == 0 {
            let monkey_true = monkeys[i].monkey_for_true;
            monkeys[monkey_true].items.push(new_item);
        } else {
            let monkey_false = monkeys[i].monkey_for_false;
            monkeys[monkey_false].items.push(new_item);
        }
    }
}

fn check_divisors(monkeys: &mut Vec<Monkey>) -> i64 {
    let lcm = monkeys
        .iter()
        .map(|m| m.test_divisor)
        .fold(1, num::integer::lcm);
    lcm
}

fn build_monkey(s: &str) -> Monkey {
    let lines = s.lines().collect::<Vec<_>>();
    let items = lines[1]
        .split_once(": ")
        .unwrap()
        .1.split(", ")
        .map(|value| value.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let operation = if
        let Ok((op_char, op_value)) = sscanf!(lines[2], "  Operation: new = old {char} {i64}")
    {
        match op_char {
            '*' => Operation::Mul(op_value),
            '+' => Operation::Add(op_value),
            _ => panic!("Unknown operation"),
        }
    } else {
        match sscanf!(lines[2], "  Operation: new = old {char} old").unwrap() {
            '*' => Operation::MulSelf,
            '+' => Operation::AddSelf,
            _ => panic!("Unknown operation"),
        }
    };
    Monkey {
        items,
        operation,
        test_divisor: sscanf!(lines[3], "  Test: divisible by {i64}").unwrap(),
        monkey_for_true: sscanf!(lines[4], "    If true: throw to monkey {usize}").unwrap(),
        monkey_for_false: sscanf!(lines[5], "    If false: throw to monkey {usize}").unwrap(),
        counter: 0,
    }
}