use aoc2022::util::read_line;

fn main() {
    let lines = read_line("./input/p9.txt").unwrap();

    let result = solve(&lines);
    println!("Part 1: {}", result);
    // println!("Part 2: {}", max);
}

fn solve(lines: &str) -> i32 {
    let head = (0, 0);
    for line in lines.lines() {
        let (dir, n) = line.split_once(" ").unwrap();
        let n: u32 = n.parse().unwrap();

        for _ in 0..n {
            head = ;
        }
    }
    0
}