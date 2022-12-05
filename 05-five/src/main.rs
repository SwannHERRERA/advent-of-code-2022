#[macro_use]
extern crate scan_fmt;

mod error;
mod prelude;

use crate::prelude::*;
use std::{collections::VecDeque, fs};

#[derive(Debug, Eq, PartialEq)]
struct Move {
    pub source: usize,
    pub destination: usize,
    pub quantity: usize,
}

fn main() -> Result<()> {
    let input = fs::read_to_string("05-five/input.txt")?;
    let res1 = part_one(&input)?;
    println!("{res1}");
    let res2 = part_two(&input)?;
    println!("part two : {res2}");
    Ok(())
}

fn part_two(input: &str) -> Result<String> {
    let (input_stacks, moves) = input.split_once("\n\n").unwrap();
    let mut input_stacks = parse_stacks_as_string(input_stacks);
    let moves: Vec<Move> = parse_moves(moves);
    execute_moves_on_vec_for_part2(&moves, &mut input_stacks);
    let code = get_last_letters(&mut input_stacks);
    Ok(code)
}

fn part_one(input: &str) -> Result<String> {
    let (input_stacks, moves) = input.split_once("\n\n").unwrap();
    // println!("{:?} {:?}", input_stacks, moves);
    let mut input_stacks = parse_stacks_as_string(input_stacks);
    let moves: Vec<Move> = parse_moves(moves);
    execute_moves_on_vec(&moves, &mut input_stacks);
    // println!("{:?}", input_stacks);
    let code = get_last_letters(&mut input_stacks);
    Ok(code)
}

fn get_last_letters(stacks: &mut Vec<VecDeque<char>>) -> String {
    let mut s = String::new();
    for stack in stacks {
        s.push(stack.pop_front().unwrap());
    }
    s
}

fn parse_stacks_as_string(input_stacks: &str) -> Vec<VecDeque<char>> {
    let mut result: Vec<VecDeque<char>> = Vec::new();

    for line in input_stacks.lines() {
        let number_of_stack = line.len() / 3;
        let mut line = line.to_string();

        for i in 0..number_of_stack {
            let char_at_center = line.clone().chars().nth(1).unwrap_or(' ');
            if char_at_center.is_uppercase() {
                while i >= result.len() {
                    result.push(VecDeque::new());
                }
                result.get_mut(i).unwrap().push_back(char_at_center);
            }
            let line_length = line.len();
            if line_length < 4 {
                line = line.split_off(line_length);
            } else {
                line = line.split_off(4);
            }
        }
    }
    result
}

fn parse_moves(moves: &str) -> Vec<Move> {
    moves
        .lines()
        .map(|line| {
            println!("{line}");
            let (quantity, source, destination) =
                scan_fmt!(line, "move {} from {} to {}", usize, usize, usize).unwrap_or((0, 0, 0));
            let (source, destination) = (source - 1, destination - 1);
            Move {
                quantity,
                source,
                destination,
            }
        })
        .collect()
}

fn execute_moves_on_vec(moves: &Vec<Move>, input_stacks: &mut Vec<VecDeque<char>>) {
    for element in moves {
        for _quantity in 0..element.quantity {
            let source = input_stacks.get_mut(element.source).unwrap();
            let value = source.pop_front().unwrap();
            drop(source);
            let dest = input_stacks.get_mut(element.destination).unwrap();
            dest.push_front(value);
        }
    }
}

fn execute_moves_on_vec_for_part2(moves: &Vec<Move>, input_stacks: &mut Vec<VecDeque<char>>) {
    for element in moves {
        if element.quantity == 1 {
            let source = input_stacks.get_mut(element.source).unwrap();
            let value = source.pop_front().unwrap();
            drop(source);
            let dest = input_stacks.get_mut(element.destination).unwrap();
            dest.push_front(value);
        } else {
            let mut input_copy = input_stacks.clone();
            let iter = (0..element.quantity).map(|_| {
                let source = input_copy.get_mut(element.source).unwrap();
                let value = source.pop_front().unwrap();
                drop(source);
                value
            }).rev();
            for el in iter {
                let dest = input_stacks.get_mut(element.destination).unwrap();
                dest.push_front(el);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use test_utils::vec_eq;

    use super::*;

    const INPUT: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part_one() {
        const EXPECTED_OUTPUT: &str = "MCD";
        let result = part_one(INPUT).unwrap();
        assert_eq!(EXPECTED_OUTPUT, result);
    }

    #[test]
    fn test_parse_move() {
        const INPUT: &str = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let expected = vec![
            Move {
                source: 1,
                destination: 0,
                quantity: 1,
            },
            Move {
                source: 0,
                destination: 2,
                quantity: 3,
            },
            Move {
                source: 1,
                destination: 0,
                quantity: 2,
            },
            Move {
                source: 0,
                destination: 1,
                quantity: 1,
            },
        ];

        let result = parse_moves(INPUT);
        println!("{:?}\n{:?}", expected, result);
        assert!(vec_eq(result, expected));
    }

    #[test]
    fn test_parse_stack_as_string() {
        const INPUT: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";
        let result = parse_stacks_as_string(INPUT);

        let mut stack1 = VecDeque::new();
        stack1.push_back('N');
        stack1.push_back('Z');

        let mut stack2 = VecDeque::new();
        stack2.push_back('D');
        stack2.push_back('C');
        stack2.push_back('M');

        let mut stack3 = VecDeque::new();
        stack3.push_back('P');

        let expected_output = vec![stack1, stack2, stack3];
        // println!("{:?}\n{:?}", expected_output, result);
        assert_eq!(expected_output.len(), result.len());
        for (index, stack) in expected_output.iter().enumerate() {
            for (expected, candidate) in stack.iter().zip(result[index].clone()) {
                assert_eq!(*expected, candidate);
            }
        }
    }
}
