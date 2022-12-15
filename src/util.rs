use std::fs::{ self, File };
use std::io::{ self, BufRead, Lines, Result, BufReader };
use std::path::Path;
use std::{ collections::HashSet };

pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_line<P>(filename: P) -> Result<String> where P: AsRef<Path> {
    let line = fs::read_to_string(filename)?;
    Ok(line)
}

pub fn print_coords(coords: &HashSet<(i32, i32)>, symbol: &str) {
    let iter_x = coords.iter().map(|c| { c.0 });
    let iter_y = coords.iter().map(|c| { c.1 });

    let min_x = iter_x.clone().min().unwrap();
    let max_x = iter_x.clone().max().unwrap();
    let min_y = iter_y.clone().min().unwrap();
    let max_y = iter_y.clone().max().unwrap();

    println!("Coords {} {} {} {}", min_x, max_x, min_y, max_y);
    println!("{:?}", coords);
    for y in min_y..=max_y {
        print!("{:02} -> ", y);
        for x in min_x..=max_x {
            if coords.contains(&(x, y)) {
                print!("{}", symbol);
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

pub fn print_coords_range(
    coords: &HashSet<(i32, i32)>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    symbol: &str
) {
    println!("Coords {} {} {} {}", min_x, max_x, min_y, max_y);
    println!("{:?}", coords);
    for y in min_y..=max_y {
        print!("{:02} -> ", y);
        for x in min_x..=max_x {
            if coords.contains(&(x, y)) {
                print!("{}", symbol);
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}