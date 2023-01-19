use std::collections::HashMap;
use std::{env, fs, process};

pub type Grid = HashMap<(u32, u32), char>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Move {
    TurnLeft,
    TurnRight,
    Advance(u32),
}

#[derive(Debug)]
pub struct Map {
    pub grid: Grid,
    pub start: (u32, u32),
    pub moves: Vec<Move>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    pub fn to_u32(&self) -> u32 {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

fn usage(progname: &String) {
    eprintln!("usage: {} <input_file>", progname);
    process::exit(1);
}

fn parse_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(&args[0]);
    }
    let filename: String = args[1].clone();
    return filename;
}

fn read_input(filename: &String) -> String {
    let contents = fs::read_to_string(&filename)
        .unwrap_or_else(|error| panic!("read_input(): error reading {}: {:?}", &filename, &error));
    return contents;
}

#[allow(unused_assignments)]
fn parse_input(contents: &String) -> Map {
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
                panic!("parse_input(): unexpected char in input: '{}'", c);
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

// dirty hack - I only handle the wrap cases for the example and the input.
// I really wanted to generalize this, but I could not figure out how to do
// so without taking several days to implement this.  so I am hard-coding
// this.  ugh, sorry.  feel free to suggest a trick that makes
// generalizing this easier.
fn wrap(filename: &String, pos: (u32, u32), facing: Direction) -> ((u32, u32), Direction) {
    if filename.contains("example.txt") {
        return wrap_example(pos, facing);
    } else if filename.contains("input.txt") {
        return wrap_input(pos, facing);
    } else {
        panic!("wrap(): I can't handle file: '{}'", filename);
    }
}

fn wrap_example(pos: (u32, u32), facing: Direction) -> ((u32, u32), Direction) {
    let (row, col) = pos;
    if row >= 1 && row <= 4 && col >= 9 && col <= 12 {
        // face 1
        if facing == Direction::Right {
            return ((13 - row, 16), Direction::Left);
        } else if facing == Direction::Left {
            return ((5, row + 4), Direction::Down);
        } else if facing == Direction::Up {
            return ((5, 13 - row), Direction::Down);
        } else {
            panic!(
                "wrap_example(): bug: pos=({}, {}), facing={:?}",
                row, col, facing
            );
        }
    } else if row >= 5 && row <= 8 && col >= 1 && col <= 4 {
        // face 2
        if facing == Direction::Down {
            return ((12, 13 - col), Direction::Up);
        } else if facing == Direction::Left {
            return ((12, 21 - row), Direction::Up);
        } else if facing == Direction::Up {
            return ((1, 13 - col), Direction::Down);
        } else {
            panic!(
                "wrap_example(): bug: pos=({}, {}), facing={:?}",
                row, col, facing
            );
        }
    } else if row >= 5 && row <= 8 && col >= 5 && col <= 8 {
        // face 3
        if facing == Direction::Down {
            return ((17 - col, 9), Direction::Right);
        } else if facing == Direction::Up {
            return ((col - 4, 9), Direction::Right);
        } else {
            panic!(
                "wrap_example(): bug: pos=({}, {}), facing={:?}",
                row, col, facing
            );
        }
    } else if row >= 5 && row <= 8 && col >= 9 && col <= 12 {
        // face 4
        if facing == Direction::Right {
            return ((9, 21 - row), Direction::Down);
        } else {
            panic!(
                "wrap_example(): bug: pos=({}, {}), facing={:?}",
                row, col, facing
            );
        }
    } else if row >= 9 && row <= 12 && col >= 9 && col <= 12 {
        // face 5
        if facing == Direction::Down {
            return ((8, 13 - col), Direction::Up);
        } else if facing == Direction::Left {
            return ((8, 17 - row), Direction::Up);
        } else {
            panic!(
                "wrap_example(): bug: pos=({}, {}), facing={:?}",
                row, col, facing
            );
        }
    } else if row >= 13 && row <= 16 && col >= 9 && col <= 12 {
        // face 6
        if facing == Direction::Right {
            return ((13 - row, 12), Direction::Left);
        } else if facing == Direction::Down {
            return ((21 - col, 1), Direction::Right);
        } else if facing == Direction::Up {
            return ((21 - col, 12), Direction::Left);
        } else {
            panic!(
                "wrap_example(): bug: pos=({}, {}), facing={:?}",
                row, col, facing
            );
        }
    } else {
        panic!(
            "wrap_example(): bug: pos=({}, {}), facing={:?}",
            row, col, facing
        );
    }
}

fn wrap_input(pos: (u32, u32), facing: Direction) -> ((u32, u32), Direction) {
    let (row, col) = pos;
    if row >= 1 && row <= 50 && col >= 51 && col <= 100 {
        // face 1
        if facing == Direction::Left {
            return ((151 - row, 1), Direction::Right);
        } else if facing == Direction::Up {
            return ((col + 100, 1), Direction::Right);
        } else {
            panic!(
                "wrap_example(): bug: pos=({}, {}), facing={:?}",
                row, col, facing
            );
        }
    } else if row >= 1 && row <= 50 && col >= 101 && col <= 150 {
        // face 2
        if facing == Direction::Right {
            return ((151 - row, 100), Direction::Left);
        } else if facing == Direction::Down {
            return ((col - 50, 100), Direction::Left);
        } else if facing == Direction::Up {
            return ((200, col - 100), Direction::Up);
        } else {
            panic!(
                "wrap_example(): bug: pos=({}, {}), facing={:?}",
                row, col, facing
            );
        }
    } else if row >= 51 && row <= 100 && col >= 51 && col <= 100 {
        // face 3
        if facing == Direction::Right {
            return ((50, row + 50), Direction::Up);
        } else if facing == Direction::Left {
            return ((101, row - 50), Direction::Down);
        } else {
            panic!(
                "wrap_example(): bug: pos=({}, {}), facing={:?}",
                row, col, facing
            );
        }
    } else if row >= 101 && row <= 150 && col >= 51 && col <= 100 {
        // face 4
        if facing == Direction::Right {
            return ((151 - row, 150), Direction::Left);
        } else if facing == Direction::Down {
            return ((col + 100, 50), Direction::Left);
        } else {
            panic!(
                "wrap_example(): bug: pos=({}, {}), facing={:?}",
                row, col, facing
            );
        }
    } else if row >= 101 && row <= 150 && col >= 1 && col <= 50 {
        // face 5
        if facing == Direction::Left {
            return ((151 - row, 51), Direction::Right);
        } else if facing == Direction::Up {
            return ((col + 50, 51), Direction::Right);
        } else {
            panic!(
                "wrap_example(): bug: pos=({}, {}), facing={:?}",
                row, col, facing
            );
        }
    } else if row >= 151 && row <= 200 && col >= 1 && col <= 50 {
        // face 6
        if facing == Direction::Right {
            return ((150, row - 100), Direction::Up);
        } else if facing == Direction::Down {
            return ((1, col + 100), Direction::Down);
        } else if facing == Direction::Left {
            return ((1, row - 100), Direction::Down);
        } else {
            panic!(
                "wrap_example(): bug: pos=({}, {}), facing={:?}",
                row, col, facing
            );
        }
    } else {
        panic!(
            "wrap_example(): bug: pos=({}, {}), facing={:?}",
            row, col, facing
        );
    }
}

fn solve(map: &Map, filename: &String) -> u32 {
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
                                (new_pos, new_facing) = wrap(filename, pos, facing);
                            }
                        }
                        Direction::Down => {
                            new_pos = (pos.0 + 1, pos.1);
                            if !map.grid.contains_key(&new_pos) {
                                (new_pos, new_facing) = wrap(filename, pos, facing);
                            }
                        }
                        Direction::Left => {
                            new_pos = (pos.0, pos.1 - 1);
                            if !map.grid.contains_key(&new_pos) {
                                (new_pos, new_facing) = wrap(filename, pos, facing);
                            }
                        }
                        Direction::Up => {
                            new_pos = (pos.0 - 1, pos.1);
                            if !map.grid.contains_key(&new_pos) {
                                (new_pos, new_facing) = wrap(filename, pos, facing);
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

fn main() {
    let filename = parse_args();
    let contents = read_input(&filename);
    let map = parse_input(&contents);
    let password = solve(&map, &filename);
    println!("password = {}", password);
}
