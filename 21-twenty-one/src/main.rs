use std::{collections::HashMap, fs};

use parse_display::{Display, FromStr};

#[derive(Display, Debug, FromStr)]
pub enum Expression {
    #[display(" {0}")]
    Integer(i64),
    #[display(" {0} + {1}")]
    Add(String, String),
    #[display(" {0} - {1}")]
    Substract(String, String),
    #[display(" {0} * {1}")]
    Multiply(String, String),
    #[display(" {0} / {1}")]
    Divide(String, String),
    #[display(" {0} = {1}")]
    Equals(String, String),
    Unknown,
}

fn main() {
   let input = fs::read_to_string("21-twenty-one/input.txt").unwrap();
   let part_one = part_one(&input);
   println!("part one : {part_one}");
   let part_two = part_two(&input);
   println!("part two : {part_two}");
}

fn part_one(input: &str) -> i64 {
    let dict = parse_input(input);
    eval_recu("root", &dict)
}

fn part_two(input: &str) -> i64 {
    let mut dict = parse_input(input);
    let root = "root".to_string();
    let root_exp = dict.get(&root).unwrap();
    match root_exp {
        Expression::Add(op1, op2) | Expression::Substract(op1, op2) | Expression::Multiply(op1, op2) | Expression::Divide(op1, op2) => {
            let new_exp = Expression::Equals(op1.clone(), op2.clone());
            dict.insert(root, new_exp);
        }
        _ => {
            unreachable!();
        }
    }
    let humn = String::from("humn");
    dict.insert(humn, Expression::Unknown);
    solve("root", &dict)
}

fn solve(name: &str, dict: &HashMap<String, Expression>) -> i64 {
    let exp = dict.get(name).unwrap();
    if let Expression::Equals(op1, op2) = exp {
        let eval_left = test_eval(op1, dict);
        let eval_right = test_eval(&op2, dict);
        if eval_left {
            let val = eval_recu(op1, dict);
            recursive_solve(op2, val, dict)
        } else if eval_right {
            let val = eval_recu(op2, dict);
            recursive_solve(op1, val, dict)
        } else {
            unreachable!();
        }
    } else {
        panic!("expect equal found {}", name);
    }
}

fn recursive_solve(name: &str, val: i64, dict: &HashMap<String, Expression>) -> i64 {
    let exp = dict.get(name).unwrap();
    match exp {
        Expression::Integer(n) => *n,
        Expression::Add(op1, op2) => {
            let left = test_eval(op1, dict);
            let right = test_eval(op2, dict);
            if left {
                let n = eval_recu(op1, dict);
                let new_val = val - n;
                recursive_solve(op2, new_val, dict)
            } else if right {
                let n = eval_recu(op2, dict);
                let new_val = val - n;
                recursive_solve(op1, new_val, dict)
            } else {
                unreachable!();
            }
        }
        Expression::Substract(op1, op2) => {
            let left = test_eval(op1, dict);
            let right = test_eval(op2, dict);
            if left {
                let n = eval_recu(op1, dict);
                let new_val = n - val;
                recursive_solve(op2, new_val, dict)
            } else if right {
                let n = eval_recu(op2, dict);
                let new_val = val + n;
                recursive_solve(op1, new_val, dict)
            } else {
                unreachable!();
            }
        }
        Expression::Multiply(op1, op2) => {
            let eval_left = test_eval(op1, dict);
            let eval_right = test_eval(op2, dict);
            if eval_left {
                let n = eval_recu(op1, dict);
                let new_val = val / n;
                recursive_solve(op2, new_val, dict)
            } else if eval_right {
                let n = eval_recu(op2, dict);
                let new_val = val / n;
                recursive_solve(op1, new_val, dict)
            } else {
                unreachable!();
            }
        }
        Expression::Divide(op1, op2) => {
            let left = test_eval(op1, dict);
            let right = test_eval(op2, dict);
            if left {
                let n = eval_recu(op1, dict);
                let new_val = n / val;
                recursive_solve(op2, new_val, dict)
            } else if right {
                let n = eval_recu(op2, dict);
                let new_val = val * n;
                recursive_solve(op1, new_val, dict)
            } else {
                unreachable!();
            }
        }
        Expression::Unknown => val,
        Expression::Equals(_, _) => unreachable!(),
    }
}

fn parse_input(input: &str) -> HashMap<String, Expression> {
    input.lines().map(|line| {
        let (name, expression) = line.split_once(':').unwrap();
        let expression: Expression = expression.parse().unwrap();
        (name.to_string(), expression)
    }).collect()
}

fn test_eval(name: &str, symtab: &HashMap<String, Expression>) -> bool {
    let exp = symtab.get(name).unwrap();
    match exp {
        Expression::Integer(_) => true,
        Expression::Equals(_, _) => false,
        Expression::Unknown => false,
        Expression::Add(op1, op2)
        | Expression::Substract(op1, op2)
        | Expression::Multiply(op1, op2)
        | Expression::Divide(op1, op2) => test_eval(op1, symtab) & test_eval(op2, symtab),
    }
}

fn eval_recu(name: &str, symtab: &HashMap<String, Expression>) -> i64 {
    let exp = symtab.get(name).unwrap();
    match exp {
        Expression::Integer(n) => *n,
        Expression::Add(op1, op2) => eval_recu(op1, symtab) + eval_recu(op2, symtab),
        Expression::Substract(op1, op2) => eval_recu(op1, symtab) - eval_recu(op2, symtab),
        Expression::Multiply(op1, op2) => eval_recu(op1, symtab) * eval_recu(op2, symtab),
        Expression::Divide(op1, op2) => eval_recu(op1, symtab) / eval_recu(op2, symtab),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_part_one() {
        let res = part_one(INPUT);
        assert_eq!(152, res);
    }

    #[test]
    fn test_part_two() {
        let res = part_two(INPUT);
        assert_eq!(301, res);
    }
}
