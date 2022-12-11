use std::collections::VecDeque;

pub type Item = usize;

pub const ROUND_PART_ONE: usize = 20;
pub const ROUND_PART_TWO: usize = 10_000;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Value {
    Old,
    Num(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
    Add(Value, Value),
    Mult(Value, Value),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Monkey {
    pub index: usize,
    pub items: VecDeque<Item>,
    pub test_divisor: usize,
    pub dest_monkey_if_true: usize,
    pub dest_monkey_if_false: usize,
    pub operation: Operation,
    pub item_touch: usize,
}

impl Monkey {
    pub fn inspect(&mut self, item: usize, trick: Option<usize>) -> usize {
        self.item_touch += 1;
        let item = item as f64;
        let new_value = match self.operation {
            Operation::Add(x, y) => Self::handle_addtion(x, y, item),
            Operation::Mult(x, y) => Self::handle_mult(x, y, item),
        };
        if let Some(t) = trick {
            return new_value as usize % t;
        }
        new_value as usize / 3
    }

    fn handle_addtion(x: Value, y: Value, item: f64) -> f64 {
        let x = match x {
            Value::Old => item,
            Value::Num(num) => num as f64,
        };
        let y = match y {
            Value::Old => item,
            Value::Num(num) => num as f64,
        };
        x + y
    }

    fn handle_mult(x: Value, y: Value, item: f64) -> f64 {
        let x = match x {
            Value::Old => item,
            Value::Num(num) => num as f64,
        };
        let y = match y {
            Value::Old => item,
            Value::Num(num) => num as f64,
        };
        x * y
    }

    pub fn process_monkey_dest(&self, value: usize) -> usize {
        match value % self.test_divisor == 0 {
            true => self.dest_monkey_if_true,
            false => self.dest_monkey_if_false,
        }
    }
}
