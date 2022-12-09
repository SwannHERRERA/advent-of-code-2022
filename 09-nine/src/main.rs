use std::fs;

use crate::{rope::Rope, types::Insctructions, utils::parse_instruction};

mod rope;
mod types;
mod utils;

fn main() {
    let input = fs::read_to_string("09-nine/input.txt").unwrap();
    let instructions: Insctructions = input
        .lines()
        .map(|line| {
            let instruction = line.split_once(' ').unwrap();
            parse_instruction(instruction)
        })
        .collect();
    let mut rope = Rope::new(2);
    rope.process(&instructions);
    let part_one = rope.cell_discovered_by_the_tail();
    let mut rope = Rope::new(10);
    rope.process(&instructions);
    let part_two = rope.cell_discovered_by_the_tail();
    println!("part one : {}", part_one);
    println!("part two : {}", part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        const INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let instructions: Insctructions = INPUT
            .lines()
            .map(|line| {
                let instruction = line.split_once(' ').unwrap();
                parse_instruction(instruction)
            })
            .collect();
        let mut rope = Rope::new(10);
        rope.process(&instructions);
        let res = rope.cell_discovered_by_the_tail();
        assert_eq!(36, res);
    }

    #[test]
    fn test_part_one() {
        const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let instructions: Insctructions = INPUT
            .lines()
            .map(|line| {
                let instruction = line.split_once(' ').unwrap();
                parse_instruction(instruction)
            })
            .collect();
        let mut rope = Rope::new(2);
        rope.process(&instructions);
        let res = rope.cell_discovered_by_the_tail();
        assert_eq!(13, res);
    }
}
