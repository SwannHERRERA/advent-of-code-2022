use std::{fs, collections::VecDeque, cmp::Ordering::*};

use itertools::Itertools;
use value::Value;

mod value;

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
    let six = Value::List(vec![Value::List(vec![Value::Num(6)])]);
    let two = Value::List(vec![Value::List(vec![Value::Num(2)])]);
    let mut values: Vec<Value> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_value(line.to_string()))
        .collect();
    values.push(six.clone());
    values.push(two.clone());
    values.sort();
    let pos_two = values.iter().find_position(|x| **x == two).unwrap();
    let pos_six = values.iter().find_position(|x| **x == six).unwrap();
    (pos_two.0 + 1) * (pos_six.0 + 1)
}

fn parse_pair(input: &str) -> [Value;2] {
    let (line_1, line_2) = input.split_once('\n').unwrap();
    let line_1 = parse_value(line_1.to_string());
    // println!("-------------------");
    let line_2 = parse_value(line_2.to_string());
    match (line_1, line_2) {
        (Value::List(list1), Value::List(list2)) => [list1[0].clone(), list2[0].clone()],
        _ => unreachable!(),
    }
}

fn parse_value(line: String) -> Value {
    const BLOCKING_CHARS: [char; 3] = ['[', ']', ','];
    if line.is_empty() {
        return Value::List(Vec::new());
    }
    let mut list = Vec::new();
    let mut chars: VecDeque<char> = line.chars().collect();
    loop {
        if chars.is_empty() {
            return Value::List(list);
        }
        let end: usize = chars.iter().take_while(|c| !BLOCKING_CHARS.contains(c)).count();
        let str: String = chars.drain(0..end).collect();
        if !str.is_empty() {
            if let Ok(value) = str.parse() {
                list.push(Value::Num(value));
            }
        }
        if let Some(blocking_char) = chars.pop_front() {
            if blocking_char == '[' {
                let new_line: String = chars.iter().collect();
                let closing_tag_index = find_coresponding_bracket(&new_line);
                let (part, _) = new_line.split_at(closing_tag_index);
                list.push(parse_value(part.to_string()));
                chars = chars.iter().skip(closing_tag_index).copied().collect();
            }
        };
    }
}

fn find_coresponding_bracket(line: &str) -> usize {
    let mut count_bracket = 0;
    for (i, c) in line.chars().enumerate() {
        if count_bracket == 0 && c == ']' {
            return i;
        }
        if c == '[' {
            count_bracket += 1;
        }
        if c == ']' {
            count_bracket -= 1;
        }
    }
    unreachable!();
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
        dbg!(&result);
        assert!(vec_eq(expected.to_vec(), result.to_vec()));
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
        dbg!(&result, &expected);

        assert!(vec_eq(expected.to_vec(), result.to_vec()));
    }

    // Should not panic
    #[test]
    fn test_advanced_parsing() {
        const INPUT: &str = "[[1,[0,3,5,[2,1,3,3,5]],4,[[],5]],[],[0,[7,[5],7,7]]]
[[[],[[],[5,2,8,9,7],1,5],[3,[]]]]";
        let result = parse_pair(INPUT);
        dbg!(result);
    }

    #[test]
    fn test_empty_arrays() {
        const INPUT: &str = "[[[]]]
[[]]";
        let result = parse_pair(INPUT);
        dbg!(&result);
        assert!(result[0] > result[1]);
    }

    #[test]
    fn test_part_two() {
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
        assert_eq!(140, part_two(INPUT));
    }
}
