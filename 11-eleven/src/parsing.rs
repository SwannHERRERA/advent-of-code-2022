use std::collections::VecDeque;

use nom::{
    bytes::complete::{tag, take, take_until},
    IResult,
};

use crate::types::{Monkey, Operation, Value};

pub fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let monkeys = input
        .split("\n\n")
        .map(parse_monkey)
        .map(|x| x.unwrap())
        .map(|x| x.1)
        .collect();
    monkeys
}

fn parse_operation(candidate: &str) -> IResult<&str, Operation> {
    let (candidate, _) = tag("new = ")(candidate)?;
    let (candidate, first) = take_until(" ")(candidate)?;
    let x = if first == "old" {
        Value::Old
    } else {
        Value::Num(first.parse().unwrap())
    };
    let (candidate, _) = take(1usize)(candidate)?; // remove space
    let (candidate, operation) = take(1usize)(candidate)?;
    let (last, _) = take(1usize)(candidate)?;

    println!("{:?}", operation);
    let y = if last == "old" {
        Value::Old
    } else {
        Value::Num(last.parse().unwrap())
    };
    Ok((
        candidate,
        match operation {
            "*" => Operation::Mult(x, y),
            "+" => Operation::Add(x, y),
            _ => unimplemented!(),
        },
    ))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (number, _) = tag("Monkey ")(input)?;
    let (input, number) = take(1usize)(number)?;
    let number: usize = number.parse().unwrap();
    let (items, _) = tag(":\n  Starting items: ")(input)?;
    let (input, array) = take_until("\n")(items)?;
    let (input, _) = tag("\n  Operation: ")(input)?;
    let starting_items: VecDeque<usize> = array.split(", ").map(|x| x.parse().unwrap()).collect();
    let (input, operation) = take_until("\n")(input)?;
    let operation = parse_operation(operation).unwrap().1;
    let (input, _) = tag("\n  Test: divisible by ")(input)?;
    let (input, divisor) = take_until("\n")(input)?;
    let divisor: usize = divisor.parse().unwrap();
    let (input, _) = tag("\n    If true: throw to monkey ")(input)?;
    let (input, if_true_monkey_dest) = take_until("\n")(input)?;
    let if_true: usize = if_true_monkey_dest.parse().unwrap();
    let (if_false, input) = tag("\n    If false: throw to monkey ")(input)?;
    let (_, if_false) = take(1usize)(if_false)?;
    let if_false: usize = if_false.parse().unwrap();
    Ok((
        input,
        Monkey {
            index: number,
            items: starting_items,
            test_divisor: divisor,
            dest_monkey_if_true: if_true,
            dest_monkey_if_false: if_false,
            operation,
            item_touch: 0,
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::tests::INPUT;

    use super::*;

    #[test]
    fn test_parse_monkeys() {
        let expected_monkeys = vec![
            Monkey {
                index: 0,
                items: vec![79, 98].into(),
                test_divisor: 23,
                dest_monkey_if_true: 2,
                dest_monkey_if_false: 3,
                operation: Operation::Mult(Value::Old, Value::Num(19)),
                item_touch: 0,
            },
            Monkey {
                index: 1,
                items: vec![54, 65, 75, 74].into(),
                test_divisor: 19,
                dest_monkey_if_true: 2,
                dest_monkey_if_false: 0,
                operation: Operation::Add(Value::Old, Value::Num(6)),
                item_touch: 0,
            },
            Monkey {
                index: 2,
                items: vec![79, 60, 97].into(),
                test_divisor: 13,
                dest_monkey_if_true: 1,
                dest_monkey_if_false: 3,
                operation: Operation::Mult(Value::Old, Value::Old),
                item_touch: 0,
            },
            Monkey {
                index: 3,
                items: vec![74].into(),
                test_divisor: 17,
                dest_monkey_if_true: 0,
                dest_monkey_if_false: 1,
                operation: Operation::Add(Value::Old, Value::Num(3)),
                item_touch: 0,
            },
        ];
        let expected_group = expected_monkeys;
        let result = parse_monkeys(INPUT);
        assert_eq!(expected_group, result);
    }

    #[test]
    fn test_parse_monkey() {
        const MONKEY: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
    ";
        let expected_monkey = Monkey {
            index: 0,
            items: vec![79, 98].into(),
            test_divisor: 23,
            dest_monkey_if_true: 2,
            dest_monkey_if_false: 3,
            operation: Operation::Mult(Value::Old, Value::Num(19)),
            item_touch: 0,
        };
        let result = parse_monkey(MONKEY);
        let (_, monkey) = result.unwrap();
        assert_eq!(expected_monkey, monkey);
    }
}
