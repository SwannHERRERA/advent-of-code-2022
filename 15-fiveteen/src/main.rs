use itertools::Itertools;
use sensor::{parse_input, sensor_dist, sensor_cover, sensor_coverage_at_y, build_ranges_by_line, find_hidden_beacon};
use types::{IndexedRanges, Point};
#[macro_use]
extern crate scan_fmt;

use std::fs;

mod types;
mod utils;
mod sensor;

fn main() {
    let input = fs::read_to_string("15-fiveteen/input.txt").unwrap();
    let part_one = part_one(&input, 2_000_000);
    println!("part one : {}", part_one);
    let part_two = part_two(&input, [0, 4_000_000, 0, 4_000_000]);
    println!("part two : {}", part_two);
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

fn part_two(input: &str, limits: [isize; 4]) -> isize {
    let plan = parse_input(input);
    // draw_grid(&plan);
    let sensors = sensor_dist(&plan);

    let ranges_by_y_index: IndexedRanges = build_ranges_by_line(plan, sensors, limits);
    let (x, y) = find_hidden_beacon(ranges_by_y_index);
    x * 4_000_000 + y
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
    fn test_part_two() {
        let res = part_two(INPUT, [0,20,0,20]);
        assert_eq!(56_000_011, res);
    }
}

