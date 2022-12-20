use std::collections::HashMap;

use crate::{types::Point, sensor::{sensor_dist, beacons}};


#[allow(unused)]
pub fn draw_grid(input: &HashMap<Point, Point>) {
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
