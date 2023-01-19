use std::collections::HashMap;
use std::fs;

use types::{Direction, Grid, Map, Move};

mod types;
fn main() {
    let input = fs::read_to_string("22-twenty-two/input.txt").unwrap();
    let part_two = part_two(&input);
    println!("part two {}", part_two);
}

fn part_one(input: &str) -> i64 {
    // broken
    // TODO FIX
    todo!()
}

fn part_two(input: &str) -> i64 {
    let map = parse_input(&input);
    solve(&map) as i64
}

fn parse_input(contents: &str) -> Map {
    let mut grid: Grid = Grid::new();
    let mut moves: Vec<Move> = vec![];
    let mut row: u32 = 1;
    let mut col: u32 = 1;
    let mut lines = contents.lines();
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        }
        for c in line.chars() {
            if c == ' ' {
                // ignore
            } else if c == '#' || c == '.' {
                grid.insert((row, col), c);
            } else {
                panic!("{}", c);
            }
            col += 1;
        }
        row += 1;
        col = 1;
    }
    let start = grid.keys().cloned().min().unwrap();
    let mut move_chars = lines.next().unwrap().chars();
    let mut num_str = String::new();
    let mut reading_num = false;
    loop {
        let maybe_ch = move_chars.next();
        if maybe_ch.is_none() {
            if reading_num {
                let n = num_str.parse::<u32>().unwrap();
                num_str = String::new();
                moves.push(Move::Advance(n));
            }
            break;
        }
        let ch = maybe_ch.unwrap();
        if ch == 'L' {
            if reading_num {
                let n = num_str.parse::<u32>().unwrap();
                num_str = String::new();
                moves.push(Move::Advance(n));
                reading_num = false;
            }
            moves.push(Move::TurnLeft);
        } else if ch == 'R' {
            if reading_num {
                let n = num_str.parse::<u32>().unwrap();
                num_str = String::new();
                moves.push(Move::Advance(n));
                reading_num = false;
            }
            moves.push(Move::TurnRight);
        } else if ch.is_digit(10) {
            reading_num = true;
            num_str.push(ch);
        }
    }
    return Map { grid, start, moves };
}

fn wrap_input(pos: (u32, u32), facing: Direction) -> ((u32, u32), Direction) {
    let (row, col) = pos;
    if row >= 1 && row <= 50 && col >= 51 && col <= 100 {
        // haut
        if facing == Direction::Left {
            ((151 - row, 1), Direction::Right)
        } else if facing == Direction::Up {
            ((col + 100, 1), Direction::Right)
        } else {
            panic!("pos=({}, {}), facing={:?}", row, col, facing);
        }
    } else if row >= 1 && row <= 50 && col >= 101 && col <= 150 {
        // devaut
        if facing == Direction::Right {
            ((151 - row, 100), Direction::Left)
        } else if facing == Direction::Down {
            ((col - 50, 100), Direction::Left)
        } else if facing == Direction::Up {
            ((200, col - 100), Direction::Up)
        } else {
            panic!("pos=({}, {}), facing={:?}", row, col, facing);
        }
    } else if row >= 51 && row <= 100 && col >= 51 && col <= 100 {
        // fond
        if facing == Direction::Right {
            ((50, row + 50), Direction::Up)
        } else if facing == Direction::Left {
            ((101, row - 50), Direction::Down)
        } else {
            panic!("pos=({}, {}), facing={:?}", row, col, facing);
        }
    } else if row >= 101 && row <= 150 && col >= 51 && col <= 100 {
        // cote 1j
        if facing == Direction::Right {
            ((151 - row, 150), Direction::Left)
        } else if facing == Direction::Down {
            ((col + 100, 50), Direction::Left)
        } else {
            panic!("pos=({}, {}), facing={:?}", row, col, facing);
        }
    } else if row >= 101 && row <= 150 && col >= 1 && col <= 50 {
        // cote 2
        if facing == Direction::Left {
            ((151 - row, 51), Direction::Right)
        } else if facing == Direction::Up {
            ((col + 50, 51), Direction::Right)
        } else {
            panic!("pos=({}, {}), facing={:?}", row, col, facing);
        }
    } else if row >= 151 && row <= 200 && col >= 1 && col <= 50 {
        if facing == Direction::Right {
            ((150, row - 100), Direction::Up)
        } else if facing == Direction::Down {
            ((1, col + 100), Direction::Down)
        } else if facing == Direction::Left {
            ((1, row - 100), Direction::Down)
        } else {
            panic!("pos=({}, {}), facing={:?}", row, col, facing);
        }
    } else {
        panic!("pos=({}, {}), facing={:?}", row, col, facing);
    }
}

fn solve(map: &Map) -> u32 {
    let mut pos = map.start;
    let mut new_pos: (u32, u32);
    let mut facing = Direction::Right;
    let turn: HashMap<(Move, Direction), Direction> = HashMap::from([
        ((Move::TurnRight, Direction::Right), Direction::Down),
        ((Move::TurnRight, Direction::Down), Direction::Left),
        ((Move::TurnRight, Direction::Left), Direction::Up),
        ((Move::TurnRight, Direction::Up), Direction::Right),
        ((Move::TurnLeft, Direction::Right), Direction::Up),
        ((Move::TurnLeft, Direction::Down), Direction::Right),
        ((Move::TurnLeft, Direction::Left), Direction::Down),
        ((Move::TurnLeft, Direction::Up), Direction::Left),
    ]);
    for m in map.moves.iter().cloned() {
        match m {
            Move::TurnRight | Move::TurnLeft => {
                facing = turn.get(&(m, facing)).unwrap().clone();
            }
            Move::Advance(n) => {
                for _ in 0..n {
                    let mut new_facing = facing;
                    match facing {
                        Direction::Right => {
                            new_pos = (pos.0, pos.1 + 1);
                            if !map.grid.contains_key(&new_pos) {
                                (new_pos, new_facing) = wrap_input(pos, facing);
                            }
                        }
                        Direction::Down => {
                            new_pos = (pos.0 + 1, pos.1);
                            if !map.grid.contains_key(&new_pos) {
                                (new_pos, new_facing) = wrap_input(pos, facing);
                            }
                        }
                        Direction::Left => {
                            new_pos = (pos.0, pos.1 - 1);
                            if !map.grid.contains_key(&new_pos) {
                                (new_pos, new_facing) = wrap_input(pos, facing);
                            }
                        }
                        Direction::Up => {
                            new_pos = (pos.0 - 1, pos.1);
                            if !map.grid.contains_key(&new_pos) {
                                (new_pos, new_facing) = wrap_input(pos, facing);
                            }
                        }
                    }
                    let ch = map.grid.get(&new_pos).unwrap();
                    if *ch == '.' {
                        pos = new_pos;
                        facing = new_facing;
                    } else if *ch == '#' {
                        break;
                    }
                }
            }
        }
    }
    return 1000 * pos.0 + 4 * pos.1 + facing.to_u32();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    #[ignore]
    fn test_part_one() {
        let res = part_one(INPUT);
        assert_eq!(6032, res);
    }
}
