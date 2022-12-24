use std::collections::HashMap;
use std::fs;

use anyhow::Result;

/// Rightward
type X = isize;

/// Downward
type Y = isize;

const START_POSITION: (X, Y) = (500, 0);

#[derive(PartialEq, Eq, Clone)]
enum Material {
    Rock,
    Sand,
}

fn main() {
    let input = fs::read_to_string("14-fourteen/input.txt").unwrap();
    // let part_one = part_one(&input);
    let part_two = part_two(&input);
    // println!("part one : {}", part_one);
    println!("part two : {}", part_two);
}

#[allow(unused)]
fn part_one(input: &str) -> usize {
    let mut grid = parse_input(input).unwrap();
    // draw_grid(&grid);

    let y_max = y_max(&grid);
    while let Some(new_position) = sand_fall(&grid, START_POSITION, y_max) {
        grid.insert(new_position, Material::Sand);
    }
    // draw_grid(&grid);

    grid.values().filter(|m| **m == Material::Sand).count()
}

fn part_two(input: &str) -> usize {
    let mut grid = parse_input(input).unwrap();

    let y_max = y_max(&grid);

    // bad way but it works
    let x_min = x_min(&grid) - 200;
    let x_max = x_max(&grid) + 200;

    for x in x_min..=x_max {
        grid.insert((x, y_max + 2), Material::Rock);
    }

    // draw_grid(&grid);

    let mut counter = 0;
    let y_max = y_max + 2;
    while let Some(new_position) = sand_fall(&grid, START_POSITION, y_max) {
        grid.insert(new_position, Material::Sand);
        counter += 1;
        if new_position == START_POSITION {
            break;
        }
    }
    // draw_grid(&grid);

    counter
}

fn parse_input(input: &str) -> Result<HashMap<(X, Y), Material>> {
    let mut out = HashMap::new();

    let paths: Vec<Vec<(X, Y)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|s| s.split_once(',').unwrap())
                .map(|(x, y)| {
                    let x: isize = x.parse().unwrap();
                    let y: isize = y.parse().unwrap();
                    (x, y)
                })
                .collect()
        })
        .collect();

    for path in paths {
        for window in path[..].windows(2) {
            if let [(x1, y1), (x2, y2)] = window {
                for x in *x1.min(x2)..=*x1.max(x2) {
                    for y in *y1.min(y2)..=*y1.max(y2) {
                        out.insert((x, y), Material::Rock);
                    }
                }
            }
        }
    }
    Ok(out)
}

fn sand_fall(grid: &HashMap<(X, Y), Material>, start: (X, Y), y_max: Y) -> Option<(X, Y)> {
    let mut y = start.1;
    let mut x = start.0;

    while y < y_max {
        let down = (x, y + 1);
        let left = (x - 1, y + 1);
        let right = (x + 1, y + 1);
        match (grid.get(&down), grid.get(&left), grid.get(&right)) {
            (Some(_), Some(_), Some(_)) => {
                return Some((x, y));
            }
            (None, _, _) => {
                y += 1;
            }
            (_, None, _) => {
                y += 1;
                x -= 1;
            }
            (_, _, None) => {
                y += 1;
                x += 1;
            }
        }
    }

    None
}

fn y_max(grid: &HashMap<(X, Y), Material>) -> Y {
    grid.keys().map(|(_, l)| *l).max().unwrap_or(0)
}

fn y_min(grid: &HashMap<(X, Y), Material>) -> Y {
    grid.keys().map(|(_, l)| *l).min().unwrap_or(0)
}

fn x_max(grid: &HashMap<(X, Y), Material>) -> Y {
    grid.keys().map(|(r, _)| *r).max().unwrap_or(0)
}

fn x_min(grid: &HashMap<(X, Y), Material>) -> Y {
    grid.keys().map(|(r, _)| *r).min().unwrap_or(0)
}

#[allow(unused)]
fn draw_grid(grid: &HashMap<(X, Y), Material>) {
    let x_min = x_min(grid);
    let x_max = x_max(grid);
    let y_min = y_min(grid);
    let y_max = y_max(grid);

    println!();

    for y in y_min..=y_max {
        print!("{y:3} ");
        for x in x_min..=x_max {
            print!(
                "{}",
                match grid.get(&(x, y)) {
                    Some(Material::Rock) => "#",
                    Some(Material::Sand) => "o",
                    None => ".",
                }
            )
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part_one() {
        let res = part_one(INPUT);
        assert_eq!(24, res);
    }

    #[test]
    fn test_part_two() {
        let res = part_two(INPUT);
        assert_eq!(93, res);
    }
}
