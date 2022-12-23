#[derive(Debug, PartialEq, Eq)]
enum Move {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Cells {
    Full,
    Empty,
}

use std::fs;

use Cells::*;

const NUMBER_OF_COLS: usize = 7;

type Shape<'a> = &'a [[Cells; 4]; 4];

#[derive(Debug)]
struct Tetris {
    maxs: Vec<usize>,
    shapes: Vec<[[Cells; 4]; 4]>,
}

impl Tetris {
    pub fn init() -> Self {
        let maxs = vec![0; NUMBER_OF_COLS];
        let shapes = vec![
            [
                [Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty],
                [Full, Full, Full, Full],
            ],
            [
                [Empty, Empty, Empty, Empty],
                [Empty, Full, Empty, Empty],
                [Full, Full, Full, Empty],
                [Empty, Full, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty],
                [Empty, Empty, Full, Empty],
                [Empty, Empty, Full, Empty],
                [Full, Full, Full, Empty],
            ],
            [
                [Full, Empty, Empty, Empty],
                [Full, Empty, Empty, Empty],
                [Full, Empty, Empty, Empty],
                [Full, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty],
                [Full, Full, Empty, Empty],
                [Full, Full, Empty, Empty],
            ],
        ];
        Tetris { maxs, shapes }
    }

    fn is_fixed(&self, shape: Shape, height: usize, offset: usize) -> bool {
        self.maxs.iter().any(|&x| x == height - 1)
    }
}

fn main() {
    let input = fs::read_to_string("17-seventeen/input.txt").unwrap();
    let input = input.trim();
    part_one(&input);
    println!("Hello, world!");
}

fn part_one(input: &str) -> usize {
    let tetris = Tetris::init();
    let moves = parse_input(&input);
    let mut moves_iterator = moves.iter().cycle();
    for (i, shape) in tetris.shapes.iter().cycle().enumerate() {
        if i == 2023 {
            return *tetris.maxs.iter().max().unwrap();
        }
        let action = moves_iterator.next().unwrap();
    }
    unreachable!();
}


fn move_shape(mut start: usize, length: usize, min: usize, max: usize, action: Move) -> usize {
    if action == Move::Left {
        start -= 1;
    } else {
        start += 1;
    }
    if start + length > max {
        return start - 1;
    }
    if start < min {
        return min;
    }
    start
}

fn parse_input(input: &str) -> Vec<Move> {
    input
        .chars()
        .map(|c| match c {
            '>' => Move::Right,
            '<' => Move::Left,
            x => panic!("{:?}", x),
        })
        .collect()
}
