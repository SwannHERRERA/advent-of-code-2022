use itertools::Itertools;
#[macro_use]
extern crate scan_fmt;

use std::{fs, collections::{HashMap, HashSet}, ops::RangeInclusive};


#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    fn manhattan_dist(&self, other: &Point) -> isize {
        self.x.abs_diff(other.x) as isize + self.y.abs_diff(other.y) as isize
    }
}

fn main() {
    let input = fs::read_to_string("15-fiveteen/input.txt").unwrap();
    let part_one = part_one(&input, 2_000_000);
    println!("part one : {}", part_one);
    let part_two = part_two(&input, [0, 4000000, 0, 4000000]);
    println!("part two : {}", part_two);
}

fn part_two(input: &str, limits: [isize; 4]) -> usize {
    let [xmin, xmax, ymin, ymax] = limits;
    let plan = parse_input(input);
    draw_grid(&plan);
    let sensors = sensor_dist(&plan);
    let line = (ymin..=ymax).find(|y| !sensors_cover(&sensors, *y));
    dbg!(line);
    unreachable!()
}

fn part_one(input: &str, y: isize) -> usize {
    let plan = parse_input(input);
    // draw_grid(&plan);
    let sensors = sensor_dist(&plan);
    sensors
        .iter()
        .filter_map(|(sensor, dist)| {
            if sensor_cover(*sensor, *dist, y) {
                Some(sensor_coverage_at_y(*sensor, *dist, y))
            } else {
                None
            }
        })
        .flatten()
        .unique()
        .filter(|x| {
            !plan.values().contains(&Point::new(*x, y))
        })
        .count()
}

fn parse_input(input: &str) -> HashMap<Point, Point> {
    let mut plan = HashMap::new();
    for line in input.lines() {
       let coordinate =  scan_fmt!(line, "Sensor at x={}, y={}: closest beacon is at x={}, y={}", isize, isize, isize, isize).unwrap_or((0, 0, 0, 0));
       plan.insert(Point::new(coordinate.0, coordinate.1), Point::new(coordinate.2, coordinate.3));
    }
    plan
}

fn sensors_cover(sensors: &HashMap<Point, isize>, y: isize) -> bool {
    sensors.iter().any(|(sensor, dist)| sensor_cover(*sensor, *dist, y))
}

fn sensor_coverage_at_y(sensor: Point, distance: isize, y: isize) -> RangeInclusive<isize> {
    let distance_to_line = sensor.y - y;
    let max_distance_on_line = distance - distance_to_line.abs();
    (sensor.x - max_distance_on_line) ..= sensor.x + max_distance_on_line
}

// fn sensor_coverage_on_line(sensor: Point, distance: isize, y: isize) -> Vec<RangeInclusive<isize>> {
//     let sensor_range = (sensor.y-distance)..(sensor.y+distance);
//     sensor_range.map(|y| {
//         sensor_coverage_at_y(sensor, distance, y)
//     }).collect()
// }

fn sensor_cover(sensor: Point, distance: isize, y: isize) -> bool {
    let sensor_range = (sensor.y-distance)..(sensor.y+distance);
    sensor_range.contains(&y)
}

#[allow(unused)]
fn draw_grid(input: &HashMap<Point, Point>) {
    let x_max = input.iter().map(|(a, b)| a.x.max(b.x)).max().unwrap_or(0);
    let x_min = input.iter().map(|(a, b)| a.x.min(b.x)).min().unwrap_or(0);
    let y_max = input.iter().map(|(a, b)| a.y.max(b.y)).max().unwrap_or(0);
    let y_min = input.iter().map(|(a, b)| a.y.min(b.y)).min().unwrap_or(0);

    let sensors = sensor_dist(input);
    let beacons = beacons(input);

    println!();

    for y in y_min..=y_max {
        print!("{y:3} ");
        for x in x_min..=x_max {
            let p = Point { x, y };
            if sensors.contains_key(&p) {
                print!("S");
            } else if beacons.contains(&p) {
                print!("B");
            } else if sensors.iter().any(|(k, d)| k.manhattan_dist(&p) <= *d) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn sensor_dist(input: &HashMap<Point, Point>) -> HashMap<Point, isize> {
    let mut out = HashMap::new();
    for (k, v) in input.iter() {
        out.insert(*k, k.manhattan_dist(v));
    }
    out
}

fn beacons(input: &HashMap<Point, Point>) -> HashSet<Point> {
    input.values().copied().collect()
}

fn process_tuning_frequency(point: Point) -> usize {
    point.x as usize * 4_000_000 + point.y as usize
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part_one() {
        let res = part_one(INPUT, 10);
        assert_eq!(26, res);
    }

    #[test]
    fn test_process_tuning_freqency() {
        let res = process_tuning_frequency(Point::new(14, 11));
        assert_eq!(56_000_011, res);
    }

    #[test]
    #[ignore = "skip to day 16"]
    fn test_part_two() {
        let res = part_two(INPUT, [0,20,0,20]);
        assert_eq!(56_000_011, res);
    }
}

