use crate::types::{Instruction, Quantity};

pub fn is_not_clamped<T: PartialOrd>(num: T, min: T, max: T) -> bool {
    !is_clamped(num, min, max)
}

pub fn is_clamped<T: PartialOrd>(num: T, min: T, max: T) -> bool {
    min <= num && num <= max
}

pub fn parse_instruction(instruction: (&str, &str)) -> Instruction {
    let direction = instruction.0.chars().last().unwrap();
    let quantity: Quantity = instruction.1.parse().unwrap();
    let direction = direction.into();
    (direction, quantity)
}
