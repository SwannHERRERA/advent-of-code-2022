use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("23-twenty-three/input.txt").unwrap();
    let part_one = part_one(&input);
    println!("part one {}", part_one);
    let part_two = part_two(&input);
    println!("part two {}", part_two);
}

fn part_one(input: &str) -> i64 {
    const ROUNDS: usize = 10;
    let mut elves = parse_input(input);
    print_map(&elves);
    for round_number in 0..ROUNDS {
        let next_move: HashMap<(i64, i64), (i64, i64)> = elves
            .par_iter()
            .filter(|&&pos| elve_should_move(pos, &elves))
            .filter_map(|&pos| compute_next_position(pos, &elves, round_number))
            .collect();
        next_move
            .iter()
            .filter(|(key, value)| {
                !next_move.iter().any(|(k, v)| {
                    value.0 == v.0 && value.1 == v.1 && !(k.0 == key.0 && k.1 == key.1)
                })
            })
            .for_each(|(k, v)| {
                elves.remove(k);
                elves.insert(*v);
            });
    }
    compute_score(&elves)
}

fn part_two(input: &str) -> usize {
    let mut elves = parse_input(input);
    let mut round = 0;
    loop {
        let next_move: HashMap<(i64, i64), (i64, i64)> = elves
            .par_iter()
            .filter(|&&pos| elve_should_move(pos, &elves))
            .filter_map(|&pos| compute_next_position(pos, &elves, round))
            .collect();
        if next_move.is_empty() {
            break;
        }
        next_move
            .iter()
            .filter(|(key, value)| {
                !next_move.iter().any(|(k, v)| {
                    value.0 == v.0 && value.1 == v.1 && !(k.0 == key.0 && k.1 == key.1)
                })
            })
            .for_each(|(k, v)| {
                elves.remove(k);
                elves.insert(*v);
            });
        round += 1;
    }
    round + 1
}

fn parse_input(input: &str) -> HashSet<(i64, i64)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_j, c)| *c == '#')
                .map(move |(j, _c)| (j as i64, i as i64))
        })
        .collect()
}

fn elve_should_move((x, y): (i64, i64), set: &HashSet<(i64, i64)>) -> bool {
    const NEIGHBOUR_POS: [(i64, i64); 8] = [
        (1, -1),
        (1, 0),
        (1, 1),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    NEIGHBOUR_POS
        .par_iter()
        .any(|(ax, ay)| set.contains(&(x + ax, y + ay)))
}

fn is_top_free((x, y): (i64, i64), elves: &HashSet<(i64, i64)>) -> bool {
    vec![(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)]
        .par_iter()
        .all(|e| !elves.contains(e))
}
fn is_right_free((x, y): (i64, i64), elves: &HashSet<(i64, i64)>) -> bool {
    vec![(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)]
        .par_iter()
        .all(|e| !elves.contains(e))
}
fn is_bottom_free((x, y): (i64, i64), elves: &HashSet<(i64, i64)>) -> bool {
    vec![(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)]
        .par_iter()
        .all(|e| !elves.contains(e))
}
fn is_left_free((x, y): (i64, i64), elves: &HashSet<(i64, i64)>) -> bool {
    vec![(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)]
        .par_iter()
        .all(|e| !elves.contains(e))
}

type DirectionPredicate = fn((i64, i64), &HashSet<(i64, i64)>) -> bool;

fn compute_next_position(
    (x, y): (i64, i64),
    set: &HashSet<(i64, i64)>,
    current_round: usize,
) -> Option<((i64, i64), (i64, i64))> {
    let choices: Vec<DirectionPredicate> =
        vec![is_top_free, is_bottom_free, is_left_free, is_right_free];
    let next_pos = vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];
    for i in 0..choices.len() {
        if choices[(i + current_round) % 4]((x, y), set) {
            return Some(((x, y), next_pos[(i + current_round) % 4]));
        }
    }
    None
}

fn print_map(elves: &HashSet<(i64, i64)>) {
    let max_x = elves.par_iter().map(|(x, _y)| x).max().unwrap();
    let max_y = elves.par_iter().map(|(_x, y)| y).max().unwrap();
    let min_x = elves.par_iter().map(|(x, _y)| x).min().unwrap();
    let min_y = elves.par_iter().map(|(_x, y)| y).min().unwrap();
    for y in *min_y - 1..=*max_y + 1 {
        for x in *min_x - 1..=*max_x + 1 {
            match elves.get(&(x, y)) {
                Some(_) => print!("#"),
                None => print!("."),
            }
        }
        println!("");
    }
    println!("");
}

fn compute_score(elves: &HashSet<(i64, i64)>) -> i64 {
    let max_x = elves.par_iter().map(|(x, _y)| x).max().unwrap();
    let max_y = elves.par_iter().map(|(_x, y)| y).max().unwrap();
    let min_x = elves.par_iter().map(|(x, _y)| x).min().unwrap();
    let min_y = elves.par_iter().map(|(_x, y)| y).min().unwrap();
    let mut score = 0;
    for y in *min_y..=*max_y {
        for x in *min_x..=*max_x {
            if !elves.contains(&(x, y)) {
                score += 1;
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn test_part_one() {
        let res = part_one(INPUT);
        assert_eq!(110, res);
    }

    #[test]
    fn test_part_two() {
        let res = part_two(INPUT);
        assert_eq!(20, res);
    }
}
