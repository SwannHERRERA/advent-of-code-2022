use std::collections::VecDeque;

pub type Item = usize;

pub const ROUND_PART_ONE: usize = 20;
pub const ROUND_PART_TWO: usize = 10_000;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Monkey {
    pub index: usize,
    pub items: VecDeque<Item>,
    pub test_divisor: usize,
    pub dest_monkey_if_true: usize,
    pub dest_monkey_if_false: usize,
    pub operation: String,
    pub item_touch: usize,
}

impl Monkey {
    pub fn inspect(&mut self, item: usize, trick: Option<usize>) -> usize {
        self.item_touch += 1;
        let item = item as f64;
        let new_value: f64 = match self.operation.as_str() {
            "new = old * 11" => item * 11.0,
            "new = old * 17" => item * 17.0,
            "new = old * 19" => item * 19.0,
            "new = old * old" => item * item,
            "new = old + 3" => item + 3.0,
            "new = old + 4" => item + 4.0,
            "new = old + 6" => item + 6.0,
            "new = old + 7" => item + 7.0,
            "new = old + 8" => item + 8.0,
            _ => unimplemented!(),
        };
        if let Some(t) = trick {
            return new_value as usize % t;
        }
        new_value as usize / 3
    }
    
    pub fn process_monkey_dest(&self, value: usize) -> usize {
        match value % self.test_divisor == 0 {
            true => self.dest_monkey_if_true,
            false => self.dest_monkey_if_false,
        }
    }
}

