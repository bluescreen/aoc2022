use std::{ collections::HashSet };

use aoc2022::util::read_line;
use itertools::Itertools;

fn main() {
    let lines = read_line("./input/p18.txt").unwrap();

    let result = solve(&lines);
    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
}

fn parse_coords(lines: &str) -> HashSet<(i32, i32, i32)> {
    let coords: HashSet<(i32, i32, i32)> = lines
        .lines()
        .map(|line| {
            let pt: (i32, i32, i32) = line
                .split(",")
                .map(|x| { x.parse::<i32>().unwrap() })
                .collect_tuple()
                .unwrap();
            pt
        })
        .collect::<HashSet<(i32, i32, i32)>>();
    coords
}

fn faces(p: (i32, i32, i32)) -> [(i32, i32, i32); 6] {
    let (x, y, z) = p;
    return [
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ];
}

fn surface_area(pts: &HashSet<(i32, i32, i32)>) -> i32 {
    let mut count = 0;
    let mut coords: HashSet<(i32, i32, i32)> = HashSet::new();

    for pt in pts.into_iter() {
        count += 6;
        for cpt in faces(*pt) {
            if coords.contains(&cpt) {
                count -= 2;
            }
        }

        coords.insert(*pt);
    }
    return count;
}

fn bounds(points: &HashSet<(i32, i32, i32)>) -> (i32, i32, i32, i32, i32, i32) {
    let xs: Vec<i32> = points
        .into_iter()
        .map(|&(x, _, _)| x)
        .collect();
    let ys: Vec<i32> = points
        .into_iter()
        .map(|&(_, y, _)| y)
        .collect();
    let zs: Vec<i32> = points
        .into_iter()
        .map(|&(_, _, z)| z)
        .collect();

    (
        *xs.iter().min().unwrap(),
        *xs.iter().max().unwrap(),
        *ys.iter().min().unwrap(),
        *ys.iter().max().unwrap(),
        *zs.iter().min().unwrap(),
        *zs.iter().max().unwrap(),
    )
}

fn solve(lines: &str) -> (i32, i32) {
    let coords = parse_coords(lines);
    let coords_copy = coords.clone();
    let coords_copy2 = coords.clone();

    let counts = find_surface_area(coords);

    let all_coords = collect_coords(coords_copy);

    let mut remaining = &all_coords - &coords_copy2;
    let mut points = vec![*remaining.iter().next().unwrap()];

    while points.len() > 0 {
        let pt = points.pop().unwrap();
        if remaining.contains(&pt) {
            remaining.remove(&pt);

            for cpt in faces(pt) {
                points.push(cpt);
            }
        }
    }
    let exterior_surface = counts - surface_area(&remaining);
    (counts, exterior_surface)
}

fn collect_coords(coords_copy: HashSet<(i32, i32, i32)>) -> HashSet<(i32, i32, i32)> {
    let (bx_min, bx_max, by_min, by_max, bz_min, bz_max) = bounds(&coords_copy);
    let all_coords: HashSet<(i32, i32, i32)> = (bx_min - 1..=bx_max + 2)
        .flat_map(|x| {
            (by_min - 1..=by_max + 2).flat_map(move |y| {
                (bz_min - 1..=bz_max + 2).map(move |z| (x, y, z))
            })
        })
        .collect();
    all_coords
}

fn find_surface_area(coords: HashSet<(i32, i32, i32)>) -> i32 {
    let counts = coords
        .iter()
        .map(|p| {
            let counts = faces(*p)
                .iter()
                .filter(|p| { coords.get(p).is_none() })
                .count() as i32;
            counts
        })
        .sum::<i32>();
    counts
}