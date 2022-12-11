use std::fs;

use nom::{
    IResult,
    bytes::complete::{tag, take, take_until},
};

type Item = usize;

#[derive(Debug, PartialEq, Eq)]
struct Monkey {
    index: usize,
    items: Vec<Item>,
    test_divisor: usize,
    dest_monkey_if_true: usize,
    dest_monkey_if_false: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct MonkeyGroup {
    monkeys: Vec<Monkey>,
    current_worry_level: usize,
}

const ROUND_MAX: usize = 20;

fn main() {
    let input = fs::read_to_string("11-eleven/input.txt").unwrap();
    let part_one = part_one(&input);
    println!("part one : {}", part_one);
}

fn part_one(input: &str) -> usize {
    let group = parse_monkeys(input);
    println!("{:?}", group);
    todo!()
}

fn parse_monkeys(input: &str) -> MonkeyGroup {
    let monkeys = input
        .split("\n\n")
        .map(parse_monkey)
        .map(|x| x.unwrap())
        .map(|x| x.1)
        .collect();
    MonkeyGroup { monkeys, current_worry_level: 0 }
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (number, _) = tag("Monkey ")(input)?;
    let (input, number) = take(1usize)(number)?;
    let number: usize = number.parse().unwrap();
    let (items, _) = tag(":\n  Starting items: ")(input)?;
    let (input, array) = take_until("\n")(items)?;
    let (input, _) = tag("\n  Operation:")(input)?;
    let starting_items: Vec<usize> = array.split(", ").map(|x| x.parse().unwrap()).collect();
    let (input, _operation) = take_until("\n")(input)?;
    let (input, _) = tag("\n  Test: divisible by ")(input)?;
    let (input, divisor) = take_until("\n")(input)?;
    let divisor: usize = divisor.parse().unwrap();
    let (input, _) = tag("\n    If true: throw to monkey ")(input)?;
    let (input, if_true_monkey_dest) = take_until("\n")(input)?;
    let if_true: usize = if_true_monkey_dest.parse().unwrap(); 
    let (if_false, input) = tag("\n    If false: throw to monkey ")(input)?;
    let (_, if_false) = take(1usize)(if_false)?;
    println!("{:?}", if_false);
    let if_false: usize = if_false.parse().unwrap();
    Ok((
        input,
        Monkey {
            index: number,
            items: starting_items,
            test_divisor: divisor,
            dest_monkey_if_true: if_true,
            dest_monkey_if_false: if_false,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
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
    fn test_parse_monkeys() {
        let expected_monkeys = vec![
            Monkey {
                index: 0,
                items: vec![79, 98],
                test_divisor: 23,
                dest_monkey_if_true: 2,
                dest_monkey_if_false: 3,
            },
            Monkey {
                index: 1,
                items: vec![54, 65, 75, 74],
                test_divisor: 19,
                dest_monkey_if_true: 2,
                dest_monkey_if_false: 0,
            },
            Monkey {
                index: 2,
                items: vec![79, 60, 97],
                test_divisor: 13,
                dest_monkey_if_true: 1,
                dest_monkey_if_false: 3,
            },
            Monkey {
                index: 3,
                items: vec![74],
                test_divisor: 17,
                dest_monkey_if_true: 0,
                dest_monkey_if_false: 1,
            },
        ];
        let expected_group = MonkeyGroup {
            monkeys: expected_monkeys,
            current_worry_level: 0,
        };
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
            items: vec![79, 98],
            test_divisor: 23,
            dest_monkey_if_true: 2,
            dest_monkey_if_false: 3,
        };
        let result = parse_monkey(MONKEY);
        println!("{:?}", result);
        let (_, monkey) = result.unwrap();
        assert_eq!(expected_monkey, monkey);
    }

}
