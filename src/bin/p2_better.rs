#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}
fn main() {
    let input = include_str!("../../input/p2.txt");
    let parsed: Vec<Vec<Play>> = input
        .lines()
        .map(|c|
            c
                .split(" ")
                .map(|x| match_play(x).unwrap())
                .collect()
        )
        .collect();
    let win_score: u32 = parsed
        .iter()
        .map(|c| calculate_winner(&c[0], &c[1]))
        .sum();
    let play_score: u32 = parsed
        .iter()
        .map(|c| shape_score(&c[1]))
        .sum();
    println!("Part 1 score will be {}", win_score + play_score);
    let part2: u32 = input
        .lines()
        .map(|c| part2(c))
        .sum();
    println!("Part 2 score will be {}", part2);
}

fn match_play(x: &str) -> Option<Play> {
    return match x {
        "A" | "X" => Some(Play::Rock),
        "B" | "Y" => Some(Play::Paper),
        "C" | "Z" => Some(Play::Scissors),
        _ => None,
    };
}

fn part2(x: &str) -> u32 {
    let parts: Vec<&str> = x.split(" ").collect();
    let play = match_play(&parts[0]).unwrap();
    let your_play = match parts[1] {
        "X" => match_winner(&play),
        "Y" => play,
        "Z" => match_loser(&play),
        _ => panic!(),
    };
    let shape_score = shape_score(&your_play);
    let win_score = calculate_winner(&play, &your_play);
    return shape_score + win_score;
}

fn calculate_winner(x: &Play, y: &Play) -> u32 {
    if x == y {
        return 3;
    }
    if &match_winner(y) == x {
        return 6;
    }
    return 0;
}

fn match_winner(x: &Play) -> Play {
    return match x {
        &Play::Rock => Play::Scissors,
        &Play::Paper => Play::Rock,
        &Play::Scissors => Play::Paper,
    };
}

fn match_loser(x: &Play) -> Play {
    return match x {
        &Play::Scissors => Play::Rock,
        &Play::Rock => Play::Paper,
        &Play::Paper => Play::Scissors,
    };
}

fn shape_score(x: &Play) -> u32 {
    return match x {
        Play::Paper => 2,
        Play::Rock => 1,
        Play::Scissors => 3,
    };
}