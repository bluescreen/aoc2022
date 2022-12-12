use aoc2022::util::read_line;

fn main() {
    let lines = read_line("./input/p10.txt").unwrap();

    let result = solve(&lines);
    println!("Part 1: {}", result);
    // println!("Part 2: {}", max);
}

fn get_signal(cycle: i32, x: i32) -> i32 {
    let signal = cycle * x;
    signal
}

fn next_cycle(line: &str, cycle: &mut i32, x: i32, crt_pos: &mut i32) {
    let pos = *cycle;
    let cpos = *crt_pos;

    let mut symbol = "";
    if cpos + 1 >= x && cpos + 1 < x + 3 {
        symbol = "#";
    } else {
        symbol = ".";
    }
    //println!("CYCLE {} POS {} X {} -> {}", cycle, crt_pos, x, symbol);

    print!("{}", symbol);

    *cycle += 1;
    if *crt_pos > 38 {
        print!("\n");
        *crt_pos = 0;
        // println!("CYCLE {} POS {} X {} -> {}", cycle, crt_pos, x, symbol);
    } else {
        *crt_pos += 1;
    }
}

fn solve(lines: &str) -> i32 {
    let mut x = 1;
    let mut cycle = 1;
    let mut signals: Vec<i32> = vec![];
    let mut crt_pos = 0;
    let watch_cycles: [i32; 6] = [20, 60, 100, 140, 180, 220];

    for line in lines.lines() {
        let (command, n) = line.split_once(" ").unwrap_or(("noop", ""));
        //println!("{}", line);
        match command {
            "noop" => {
                if watch_cycles.contains(&cycle) {
                    signals.push(get_signal(cycle, x));
                }

                next_cycle(line, &mut cycle, x, &mut crt_pos);
            }
            "addx" => {
                if watch_cycles.contains(&cycle) {
                    signals.push(get_signal(cycle, x));
                }
                next_cycle(line, &mut cycle, x, &mut crt_pos);
                if watch_cycles.contains(&cycle) {
                    signals.push(get_signal(cycle, x));
                }
                next_cycle(line, &mut cycle, x, &mut crt_pos);

                x += n.parse::<i32>().unwrap();
            }
            _ => {}
        }
    }
    println!("\nSignals\n{:?}", signals);

    signals.iter().sum::<i32>()
}