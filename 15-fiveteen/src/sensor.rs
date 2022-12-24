use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ops::RangeInclusive,
};

use crate::types::{IndexedRanges, MergedRange, Point};

pub fn parse_input(input: &str) -> HashMap<Point, Point> {
    let mut plan = HashMap::new();
    for line in input.lines() {
        let coordinate = scan_fmt!(
            line,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            isize,
            isize,
            isize,
            isize
        )
        .unwrap_or((0, 0, 0, 0));
        plan.insert(
            Point::new(coordinate.0, coordinate.1),
            Point::new(coordinate.2, coordinate.3),
        );
    }
    plan
}

pub fn sensor_dist(input: &HashMap<Point, Point>) -> HashMap<Point, isize> {
    let mut out = HashMap::new();
    for (k, v) in input.iter() {
        out.insert(*k, k.manhattan_dist(v));
    }
    out
}

pub fn sensor_coverage_at_y(sensor: Point, distance: isize, y: isize) -> RangeInclusive<isize> {
    let distance_to_line = sensor.y - y;
    let max_distance_on_line = distance - distance_to_line.abs();
    (sensor.x - max_distance_on_line)..=sensor.x + max_distance_on_line
}

pub fn sensor_cover(sensor: Point, distance: isize, y: isize) -> bool {
    let sensor_range = (sensor.y - distance)..(sensor.y + distance);
    sensor_range.contains(&y)
}

pub fn beacons(input: &HashMap<Point, Point>) -> HashSet<Point> {
    input.values().copied().collect()
}

pub fn build_ranges_by_line(
    plan: HashMap<Point, Point>,
    sensors: HashMap<Point, isize>,
    limits: [isize; 4],
) -> IndexedRanges {
    let [_xmin, xmax, ymin, ymax] = limits;
    plan.iter()
        .flat_map(|(sensor, _closest_beacon)| {
            let max_distance = sensors.get(sensor).unwrap();
            let ranges = sensor_coverage_on_lines(sensor.clone(), *max_distance);
            ranges
                .into_iter()
                .map(|(y, range)| (y, *range.start().max(&0)..=*range.end().min(&xmax)))
        })
        .filter(|(y, _)| *y >= ymin && *y <= ymax)
        .fold(BTreeMap::new(), |mut acc, (y, range)| {
            acc.entry(y)
                .and_modify(|ranges| ranges.push(range.clone()))
                .or_insert(vec![range]);
            acc
        })
}

pub fn find_hidden_beacon(ranges_by_y_index: IndexedRanges) -> (isize, isize) {
    ranges_by_y_index
        .into_iter()
        .find_map(|(y_index, mut ranges)| {
            ranges.sort_by(|a, b| a.start().cmp(b.start()));
            let result: MergedRange = ranges.iter().fold((0..=0, None), merge_ranges);
            result.1.map(|x| (x, y_index))
        })
        .unwrap()
}

fn merge_ranges(mut acc: MergedRange, range: &RangeInclusive<isize>) -> MergedRange {
    if acc.1.is_some() {
        return acc;
    }
    if acc.0.end() + 1 >= *range.start() {
        acc.0 = *acc.0.start()..=(*acc.0.end().max(range.end()));
    } else {
        acc.1 = Some(acc.0.end() + 1);
    }

    acc
}

fn sensor_coverage_on_lines(sensor: Point, distance: isize) -> Vec<(isize, RangeInclusive<isize>)> {
    let sensor_range = (sensor.y - distance)..(sensor.y + distance);
    sensor_range
        .map(|y| {
            let coverage = sensor_coverage_at_y(sensor, distance, y);
            (y, coverage)
        })
        .collect()
}
