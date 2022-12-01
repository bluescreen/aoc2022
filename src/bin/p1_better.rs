use aoc2022::util::read_line;

fn main() {
    let lines = read_line("./input/p1.txt").unwrap();
    let mut all_max: Vec<u32> = lines
        .split("\n\n")
        .map(|x| {
            return x
                .split("\n")
                .flat_map(|p| p.parse::<u32>())
                .sum::<u32>();
        })
        .collect();

    println!("max {:?}", all_max.iter().max().unwrap());

    all_max.sort_by(|a, b| b.cmp(a));
    println!("top three {}", all_max.iter().take(3).sum::<u32>());
}