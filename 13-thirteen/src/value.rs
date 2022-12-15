use std::{cmp::Ordering, fmt::Display};

use itertools::Itertools;

#[derive(Debug, Clone, Eq)]
pub enum Value {
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
