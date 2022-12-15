use std::fs;

use info::read_info;
use maze::Maze;

mod info;
mod maze;

fn main() {
    let input = fs::read("12-twelve/input.txt").unwrap();
    let part_one = part_one(input.clone());
    println!("part one: {}", part_one);
    let part_two = part_two(input);
    println!("part two: {}", part_two);
}

pub fn part_one(mut bytes: Vec<u8>) -> usize {
    let info = read_info(&bytes);
    bytes[info.start_pos] = b'a';
    bytes[info.end_pos] = b'z';
    let mut state = Maze::new(&bytes, info.line_length);
    state.queue.push_back((info.start_pos, info.start_pos));
    state.find_path(info.end_pos);
    state.count_steps(info.end_pos)
}

pub fn part_two(mut bytes: Vec<u8>) -> usize {
    let info = read_info(&bytes);
    bytes[info.start_pos] = b'a';
    bytes[info.end_pos] = b'z';
    let mut state = Maze::new(&bytes, info.line_length);
    for (i, &c) in bytes.iter().enumerate() {
        if c == b'a' {
            state.queue.push_back((i, i));
        }
    }
    state.find_path(info.end_pos);
    state.count_steps(info.end_pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_part_one() {
        let input: Vec<u8> = INPUT.as_bytes().to_vec();
        let res = part_one(input);
        assert_eq!(31, res);
    }

    #[test]
    fn test_part_two() {
        let input: Vec<u8> = INPUT.as_bytes().to_vec();
        let res = part_two(input);
        assert_eq!(29, res);
    }
}
