use std::cmp::min;
use std::cmp::max;
use std::collections::HashSet;

use aoc2022::util::read_line;
use itertools::Itertools;

fn main() {
    let lines = read_line("./input/p14.txt").unwrap();

    println!("Part 1: {}", solve(&lines));
    // println!("Part 2: {}", solve(lines));
}

fn solve(lines: &str) -> i32 {
    let mut coords: HashSet<(i32, i32)> = parse_coords(lines)
        .iter()
        .map(|p| { (p.x, p.y) })
        .collect();

    println!("{:#?}", coords);
    let max_y = coords
        .iter()
        .map(|p| { p.1 })
        .max()
        .unwrap();

    let mut i = 0;
    loop {
        let mut p = Point { x: 500, y: 0 };
        loop {
            println!("{:?}", p);

            if !coords.contains(&(p.x, p.y + 1)) {
                p.y += 1;
            } else if !coords.contains(&(p.x - 1, p.y + 1)) {
                p.x -= 1;
                p.y += 1;
            } else if !coords.contains(&(p.x + 1, p.y + 1)) {
                p.x += 1;
                p.y += 1;
            } else {
                coords.insert((p.x, p.y));
                break;
            }

            if p.y > max_y {
                return i;
            }
        }
        i += 1;
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn parse(coords: (&str, &str)) -> Self {
        Point {
            x: coords.0.parse::<i32>().unwrap(),
            y: coords.1.parse::<i32>().unwrap(),
        }
    }

    fn tuple(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

fn parse_coords(lines: &str) -> Vec<Point> {
    let coords = lines
        .lines()
        .flat_map(|line| {
            let coords_parts = line.split(" -> ").collect::<Vec<&str>>();
            let coords: Vec<Point> = coords_parts
                .iter()
                .map(|coord| {
                    let coords = coord.split_once(",").unwrap();
                    Point::parse(coords)
                })
                .collect();

            let connected = connect_points(coords);

            connected
        })
        .collect_vec();

    coords
}

fn connect_points(coords: Vec<Point>) -> Vec<Point> {
    let mut prev = &coords[0];
    let mut connected: Vec<Point> = vec![];
    for coord in &coords[1..] {
        if coord.x == prev.x {
            for y in min(coord.y, prev.y)..max(coord.y, prev.y) + 1 {
                connected.push(Point { x: coord.x, y });
            }
        } else {
            for x in min(coord.x, prev.x)..max(coord.x, prev.x) + 1 {
                connected.push(Point { x, y: coord.y });
            }
        }
        prev = coord;
    }
    connected
}