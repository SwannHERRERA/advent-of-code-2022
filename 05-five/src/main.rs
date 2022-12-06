#[macro_use]
extern crate scan_fmt;

mod error;
mod prelude;

use crate::prelude::*;
use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug, Eq, PartialEq)]
struct Move {
    pub source: usize,
    pub destination: usize,
    pub quantity: usize,
}

type Stacks = Vec<VecDeque<char>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("05-five/input.txt")?;
    let res1 = part_one(&input)?;
    println!("part one : {res1}");
    let res2 = part_two(&input)?;
    println!("part two : {res2}");
    Ok(())
}

fn part_one(input: &str) -> Result<String> {
    let (input_stacks, moves) = input.split_once("\n\n").unwrap();
    let mut input_stacks = parse_stacks_as_string(input_stacks);
    let moves: Vec<Move> = parse_moves(moves);
    execute_moves_on_vec(&moves, &mut input_stacks);
    let code = get_last_letters(&mut input_stacks);
    Ok(code)
}

fn part_two(input: &str) -> Result<String> {
    let (input_stacks, moves) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks_as_string(input_stacks);
    let moves: Vec<Move> = parse_moves(moves);
    execute_moves_on_vec_for_part2(&moves, &mut stacks);
    println!("final Stacks");
    debug_stack(&stacks);
    let code = get_last_letters(&mut stacks);
    Ok(code)
}

fn parse_stacks_as_string(input_stacks: &str) -> Stacks {
    const SIZE_OF_COLUMN: usize = 4;
    const MIN_SIZE_OF_COLUMN: usize = 3;
    const ITEM_INDEX: usize = 1;
    let mut result: Stacks = Vec::new();

    for line in input_stacks.lines() {
        let number_of_stack = line.len() / MIN_SIZE_OF_COLUMN;
        let mut line = line.to_string();

        for i in 0..number_of_stack {
            let char_at_center = line.clone().chars().nth(ITEM_INDEX).unwrap_or(' ');
            if char_at_center.is_uppercase() {
                fill_result(&mut result, i);
                result.get_mut(i).unwrap().push_back(char_at_center);
            }
            let line_length = line.len();
            if line_length < SIZE_OF_COLUMN {
                line = line.split_off(line_length);
            } else {
                line = line.split_off(SIZE_OF_COLUMN);
            }
        }
    }
    result
}

fn parse_moves(moves: &str) -> Vec<Move> {
    moves
        .lines()
        .map(|line| {
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

fn get_last_letters(stacks: &mut Stacks) -> String {
    let mut s = String::new();
    for stack in stacks {
        s.push(stack.pop_front().unwrap());
    }
    s
}

fn execute_moves_on_vec(moves: &Vec<Move>, stacks: &mut Stacks) {
    for element in moves {
        for _quantity in 0..element.quantity {
            let value = get_char_to_change(stacks, element.source);
            let dest = stacks.get_mut(element.destination).unwrap();
            dest.push_front(value);
        }
    }
}

fn get_char_to_change(stacks: &mut Stacks, index: usize) -> char {
    let source = stacks.get_mut(index).unwrap();
    source.pop_front().unwrap()
}

fn get_chars_to_change(stacks: &mut Stacks, quantity: usize, source: usize) -> Vec<char> {
    let stack: &mut VecDeque<char> = stacks.get_mut(source).unwrap();
    (0..quantity).map(|_| stack.pop_front().unwrap()).collect()
}

fn execute_moves_on_vec_for_part2(moves: &Vec<Move>, stacks: &mut Stacks) {
    for element in moves {
        debug_stack(stacks);
        println!("{:?}", element);
        let source = get_chars_to_change(stacks, element.quantity, element.source);
        for el in source.iter().rev() {
            let dest = stacks.get_mut(element.destination).unwrap();
            dest.push_front(*el);
        }
    }
}

fn fill_result(result: &mut Stacks, final_size: usize) {
    while final_size >= result.len() {
        result.push(VecDeque::new());
    }
}

fn debug_stack(stacks: &Stacks) {
    let Some(max_length) = stacks.iter().map(|s| s.len()).max() else {
        println!("no max length");
        return;
    };

    for i in (0..=(max_length - 1)).rev() {
        let line_content: HashMap<usize, VecDeque<char>> = stacks
            .iter()
            .enumerate()
            .filter(|(_idx, stack)| stack.len() > i)
            .map(|(idx, stack)| (idx, stack.clone()))
            .collect();

        for x in 0..stacks.len() {
            if let Some(stack) = line_content.get(&x) {
                print!("[{}] ", stack[stack.len() - i - 1]);
            } else {
                print!("    ");
            }
        }
        println!();
    }
    println!();
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
    fn test_part_two() {
        const EXPECTED_OUTPUT: &str = "MCD";
        let result = part_two(INPUT).unwrap();
        assert_eq!(EXPECTED_OUTPUT, result);
    }

    #[test]
    fn test_part_one() {
        const EXPECTED_OUTPUT: &str = "CMZ";
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
