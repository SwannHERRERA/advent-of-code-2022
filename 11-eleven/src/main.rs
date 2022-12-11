use std::fs;

use types::Monkey;

use crate::{parsing::parse_monkeys, types::{ROUND_PART_TWO, ROUND_PART_ONE}};

mod types;
mod parsing;


fn main() {
    let input = fs::read_to_string("11-eleven/input.txt").unwrap();
    let part_one = part_one(&input);
    println!("part one : {}", part_one);
    let part_two = part_two(&input);
    println!("part two : {}", part_two);
}

fn part_two(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);
    let trick = monkeys
        .iter()
        .map(|monkey| monkey.test_divisor)
        .product::<usize>();
    let trick = Some(trick);
    (0..ROUND_PART_TWO).for_each(|_| {
        exec_round(&mut monkeys, trick);
    });
    monkeys.sort_by_key(|m| m.item_touch);
    println!("{:?}", monkeys);
    monkeys.iter().rev().take(2).map(|m| m.item_touch).product()
}

fn part_one(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);
    (0..ROUND_PART_ONE).for_each(|_| {
        exec_round(&mut monkeys, None);
    });
    monkeys.sort_by_key(|m| m.item_touch);
    println!("{:?}", monkeys);
    monkeys.iter().rev().take(2).map(|m| m.item_touch).product()
}

fn exec_round(monkeys: &mut Vec<Monkey>, trick: Option<usize>) {
    (0..monkeys.len()).for_each(|monkey_index| {
        (0..monkeys[monkey_index].items.len()).for_each(|_| {
            process_round_for_an_item(monkey_index, monkeys, trick);
        });
    });
}

fn process_round_for_an_item(monkey_index: usize, monkeys: &mut Vec<Monkey>, trick: Option<usize>) {
    let monkey = monkeys.get_mut(monkey_index).unwrap();
    let item = monkey.items.pop_front().unwrap();// removing at start move back other items
    let new_item = monkey.inspect(item, trick);
    let index_monkey_dest = monkey.process_monkey_dest(new_item);
    drop(monkey);
    monkeys.get_mut(index_monkey_dest).unwrap().items.push_back(new_item);
}


#[cfg(test)]
mod tests {
    use super::*;

    pub const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
    ";

    #[test]
    fn test_part_one() {
        const EXPECTED_MONKEY_BUSINESS: usize = 10605;
        let monkey_buisiness = part_one(INPUT);
        assert_eq!(EXPECTED_MONKEY_BUSINESS, monkey_buisiness);
    }

    #[test]
    fn test_part_two() {
        const EXPECTED_MONKEY_BUSINESS: usize = 2713310158;
        let monkey_buisiness = part_two(INPUT);
        assert_eq!(EXPECTED_MONKEY_BUSINESS, monkey_buisiness);
    }


}
