use aoc2022::util::read_lines;

fn main() {
    let mut max = 0;
    let mut all_max: Vec<u32> = vec![];

    if let Ok(lines) = read_lines("./input/p1.txt") {
        for value in lines {
            let string: String = value.unwrap();
            if string == "" {
                all_max.push(max);
                max = 0;
            } else {
                let number: u32 = string.parse().expect("msg");
                max = max + number;
            }
        }
    }
    all_max.sort();
    let top_three: &[u32] = &all_max[&all_max.len() - 3..];
    let sum_top: u32 = top_three.iter().sum();

    println!("max {:?}", &all_max.iter().max().unwrap());
    println!("sum top three {:?}", sum_top);
}