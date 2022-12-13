use std::any::Any;

use aoc2022::util::read_line;
use py_literal::Value;

fn main() {
    let lines = read_line("./input/p13_test.txt").unwrap();

    let result = solve(&lines);
    println!("Part 1: {}", result);
    // println!("Part 2: {}", max);
}

fn solve(lines: &str) -> usize {
    let sum: usize = lines
        .split("\n\n")
        .take(1)
        .filter(|x| {
            let (left, right) = x.split_once("\n").unwrap();

            let left_list: Value = left.parse().unwrap();
            let right_list: Value = right.parse().unwrap();
            compare(left_list, right_list)
        })
        .count();
    sum
}

enum Ordering {
    Less,
    Greater,
    Equal,
}

fn compare(lhs: &dyn Any, rhs: &dyn Any) -> Ordering {
    if let (Some(lhs), Some(rhs)) = (lhs.downcast_ref::<i32>(), rhs.downcast_ref::<i32>()) {
        return lhs.cmp(rhs);
    }

    if let (Some(lhs), Some(rhs)) = (lhs.downcast_ref::<Vec<_>>(), rhs.downcast_ref::<Vec<_>>()) {
        for (a, b) in lhs
            .iter()
            .zip(rhs.iter())
            .take_while(|(a, b)| a.is_some() && b.is_some()) {
            match compare(a.as_ref().unwrap(), b.as_ref().unwrap()) {
                Ordering::Less => {
                    return Ordering::Less;
                }
                Ordering::Greater => {
                    return Ordering::Greater;
                }
                Ordering::Equal => {}
            }
        }
        return Ordering::Equal;
    }

    if lhs.is::<Vec<_>>() {
        return Ordering::Less;
    }

    if rhs.is::<Vec<_>>() {
        return Ordering::Greater;
    }

    panic!("unsupported type");
}