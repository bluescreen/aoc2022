use aoc2022::util::{ read_line, print_coords, print_coords_range };
use std::{ collections::HashSet };
use sscanf::sscanf;

#[derive(Debug)]
struct Sensor {
    pos_x: i32,
    pos_y: i32,
    beacon_x: i32,
    beacon_y: i32,
}
impl Sensor {
    fn distance(&self) -> i32 {
        (self.pos_x - self.beacon_x).abs() + (self.pos_y - self.beacon_y).abs()
    }
}

fn main() {
    let lines = read_line("./input/p15.txt").unwrap();

    println!("Part 1: {}", solve(&lines, 2000000));
    // println!("Part 2: {}", solve2(&lines));
}

fn solve(lines: &str, y: i32) -> i32 {
    let mut beacons = HashSet::new();
    let mut sensors = HashSet::new();
    let mut coords = HashSet::new();

    for line in lines.lines() {
        let (x1, y1, x2, y2) = sscanf!(
            line,
            "Sensor at x={i32}, y={i32}: closest beacon is at x={i32}, y={i32}"
        ).unwrap();

        let sensor = Sensor { pos_x: x1, pos_y: y1, beacon_x: x2, beacon_y: y2 };

        beacons.insert((sensor.beacon_x, sensor.beacon_y));
        sensors.insert((sensor.pos_x, sensor.pos_y));
        let dist = sensor.distance();
        let right_term = dist - (y - sensor.pos_y).abs();
        for x in sensor.pos_x - right_term..=sensor.pos_x + right_term {
            if sensor.pos_x == 8 && sensor.pos_y == 7 {
                println!("{:?}", (x, y));
            }
            coords.insert((x, y));
        }
    }
    let diff = &coords - &beacons;
    diff.len() as i32
}