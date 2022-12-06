use aoc2022::util::read_line;

fn main() {
    let lines = read_line("./input/p6.txt").unwrap();

    let result = find_start_of_packet_part1(&lines);
    println!("Part 1 {}", result);

    let result = find_start_of_packet_part2(&lines);
    println!("Part 2 {}", result);
}

fn find_start_of_packet_part1(input: &str) -> i32 {
    let mut last_four = Vec::new();
    for (i, c) in input.chars().enumerate() {
        last_four.push(c);
        if last_four.len() == 4 {
            if check_for_marker(last_four.clone()) {
                return (i as i32) + 1;
            } else {
                last_four.remove(0);
            }
        }
    }
    0
}

fn find_start_of_packet_part2(input: &str) -> i32 {
    let mut last_fourteen = Vec::new();
    for (i, c) in input.chars().enumerate() {
        last_fourteen.push(c);
        if last_fourteen.len() == 14 {
            if check_for_distinct_chars(&last_fourteen) {
                return (i as i32) + 1;
            } else {
                last_fourteen.remove(0);
            }
        }
    }
    0
}

fn check_for_marker(last_four: Vec<char>) -> bool {
    last_four[0] != last_four[1] &&
        last_four[0] != last_four[2] &&
        last_four[0] != last_four[3] &&
        last_four[1] != last_four[2] &&
        last_four[1] != last_four[3] &&
        last_four[2] != last_four[3]
}

fn check_for_distinct_chars(last_fourteen: &Vec<char>) -> bool {
    last_fourteen.iter().all(
        |&x|
            last_fourteen
                .iter()
                .filter(|&&y| x == y)
                .count() == 1
    )
}