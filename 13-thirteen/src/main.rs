use itertools::Itertools;
use std::{fs, collections::VecDeque, cmp::Ordering::{self, *}, fmt::Display};

#[derive(Debug, Clone, Eq)]
enum Value {
    Num(u8),
    List(Vec<Value>),
}

impl From<i32> for Value {
    fn from(num: i32) -> Self {
        Value::Num(num as u8)
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l), Self::List(r)) => l == r,
            (Self::Num(l), Self::Num(r)) => {
                l == r
            }
            (Self::List(l), Self::Num(r)) => {
                l == &vec![Value::Num(*r)]
            }
            (Self::Num(l), Self::List(r)) => {
                &vec![Value::Num(*l)] == r
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::List(a), Value::List(b)) => a.cmp(b),
            (Value::List(a), Value::Num(b)) => {
                a.cmp(&vec![Value::Num(*b)])
            }
            (Value::Num(a), Value::List(b)) => {
                vec![Value::Num(*a)].cmp(&b)
            }
            (Value::Num(a), Value::Num(b)) => {
                a.cmp(b)
            }
        }
    }
}

impl Display for Value {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::List(list) => format!(
                    "[{}]",
                    list.iter()
                        .map(|v| v.to_string())
                        .intersperse(",".to_string())
                        .collect::<String>()
                ),
                Value::Num(num) => num.to_string(),
            }
        )
    }
}

fn main() {
    let input = fs::read_to_string("13-thirteen/input.txt").unwrap();
    let part_one = part_one(&input);
    println!("part one : {}", part_one);
    let part_two = part_two(&input);
    println!("part two : {}", part_two);
}

fn part_one(input: &str) -> usize {
    let pairs: Vec<[Value; 2]> = input
        .split("\n\n")
        .map(|pair| parse_pair(pair))
        .collect();
    pairs
        .iter()
        .enumerate()
        .map(|(i, [l, r])| {
            println!("{}", l);
            println!("{}", r);
            println!("");
            (i, [l, r])
        })
        .filter_map(|(i, [left, right])| {
            match left.cmp(right) {
                Less => Some(i),
                Equal => unreachable!(),
                Greater => None,
            }
        })
    .map(|x| x + 1)
    .sum()
}

fn part_two(input: &str) -> usize {
    todo!()
}

fn parse_pair(input: &str) -> [Value;2] {
    println!("{input}");
    let (line_1, line_2) = input.split_once('\n').unwrap();
    let line_1 = parse_value(line_1);
    println!("-------------------");
    let line_2 = parse_value(line_2);
    [line_1, line_2]
}

fn parse_value(mut line: &str) -> Value {
    const BLOCKING_CHARS: [char; 3] = ['[', ']', ','];
    let mut list = Vec::new();
    let mut chars: VecDeque<char> = line.chars().skip(1).collect();
    loop {
        let x: usize = chars.iter().take_while(|c| !BLOCKING_CHARS.contains(c)).count();
        let str: String = chars.drain(0..x).collect();
        let Some(blocking_char) = chars.pop_front() else {
            return Value::List(list);
        };
        if !str.is_empty() {
            let value = Value::Num(str.parse().unwrap());
            list.push(value);
        }
        if blocking_char == ']' {
            return Value::List(list);
        }
        if blocking_char == '[' {
            let closing_tag = line.find(']').unwrap();
            let (part, next) = line.split_at(closing_tag+1);
            list.push(parse_value(&part[1..]));
            chars = chars.iter().skip_while(|x| **x != ']').skip(1).copied().collect();
            line = next;
        }
    }
}


#[cfg(test)]
mod tests {
    use test_utils::vec_eq;

    use super::*;

    #[test]
    fn test_part_one() {
        const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        let response = part_one(INPUT);
        assert_eq!(13, response);
    }

    #[test]
    fn test_parsing() {
        const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]";
        let expected: [Value; 2] = [
            Value::List(vec![1.into(),1.into(),3.into(),1.into(),1.into()]),
            Value::List(vec![1.into(),1.into(),5.into(),1.into(),1.into()]),
        ];
        let result = parse_pair(INPUT);
        assert!(vec_eq(expected.to_vec(), result.to_vec()));
        dbg!(result);
    }
    #[test]
    fn test_parsing_with_sub_array() {
        const INPUT: &str = "[[1],[2,3,4]]
[[1],4]";
        let expected: [Value; 2] = [
            Value::List(vec![Value::List(vec![1.into()]),Value::List(vec![2.into(), 3.into(), 4.into()])]),
            Value::List(vec![Value::List(vec![1.into()]), Value::Num(4)]),
        ];
        let result = parse_pair(INPUT);

        assert!(vec_eq(expected.to_vec(), result.to_vec()));
    }
}
