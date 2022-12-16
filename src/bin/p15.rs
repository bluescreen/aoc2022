use aoc2022::util::{ read_line, print_coords, print_coords_range };
use std::{ collections::HashSet };
use sscanf::sscanf;

#[derive(Debug)]
struct Sensor {
    pos_x: i64,
    pos_y: i64,
    beacon_x: i64,
    beacon_y: i64,
}
impl Sensor {
    fn distance(&self) -> i64 {
        (self.pos_x - self.beacon_x).abs() + (self.pos_y - self.beacon_y).abs()
    }

    fn c_dist(&self, x: i64, y: i64) -> i64 {
        (self.pos_x - x).abs() + (self.pos_y - y).abs()
    }
}

fn main() {
    let lines = read_line("./input/p15.txt").unwrap();

    //println!("Part 1: {}", solve(&lines, 2000000));
    println!("Part 2: {}", solve2(&lines, 4000000));
}

fn solve(lines: &str, y: i64) -> i64 {
    let mut beacons = HashSet::new();
    let mut coords = HashSet::new();

    for line in lines.lines() {
        let (x1, y1, x2, y2) = sscanf!(
            line,
            "Sensor at x={i64}, y={i64}: closest beacon is at x={i64}, y={i64}"
        ).unwrap();

        let sensor = Sensor { pos_x: x1, pos_y: y1, beacon_x: x2, beacon_y: y2 };

        beacons.insert((sensor.beacon_x, sensor.beacon_y));
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
    diff.len() as i64
}

fn solve2(lines: &str, m: i64) -> i64 {
    let mut beacons = HashSet::new();
    let mut sensors: Vec<Sensor> = vec![];

    for line in lines.lines() {
        let (x1, y1, x2, y2) = sscanf!(
            line,
            "Sensor at x={i64}, y={i64}: closest beacon is at x={i64}, y={i64}"
        ).unwrap();

        let sensor = Sensor { pos_x: x1, pos_y: y1, beacon_x: x2, beacon_y: y2 };

        beacons.insert((sensor.beacon_x, sensor.beacon_y));
        sensors.push(sensor);
    }

    let all_sensors = sensors;
    //println!("{:?}", all_sensors);

    for sensor in &all_sensors {
        let top_y = sensor.pos_y + sensor.distance() - 1;
        let bottom_y = sensor.pos_y - sensor.distance() - 1;

        for i in 0..sensor.distance() {
            for (x, y) in vec![
                (sensor.pos_x + i, top_y - i),
                (sensor.pos_x - i, top_y - i),
                (sensor.pos_x + i, bottom_y + i),
                (sensor.pos_x - i, bottom_y + i)
            ] {
                if x < 0 || y < 0 || x > m || y > m {
                    continue;
                } else if beacons.contains(&(x, y)) {
                    continue;
                }

                let mut found = false;
                for sensor_2 in &all_sensors {
                    if sensor_2.c_dist(x, y) <= sensor_2.distance() {
                        found = true;
                        break;
                    }
                }

                if !found {
                    return x * 4000000 + y;
                }
            }
        }
    }

    0
}