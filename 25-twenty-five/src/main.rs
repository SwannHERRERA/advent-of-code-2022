use std::fs;

fn main() {
    let input = fs::read_to_string("25-twenty-five/input.txt").unwrap();
    let part_one = part_one(&input);
    println!("part one : {}", part_one);
}

fn part_one(input: &str) -> String {
    let sum = input.lines().map(to_decimal).sum();
    to_snafu(sum)
}

fn to_snafu(decimal: i64) -> String {
    if decimal == 0 {
        return String::new();
    }
    let new_decimal = (decimal + 2) / 5;
    let mut snafu = to_snafu(new_decimal);

    let decimal_remainder = decimal % 5;
    let snafu_digit = ['0', '1', '2', '=', '-'][decimal_remainder as usize];
    snafu.push(snafu_digit);

    snafu
}

fn parse_token(c: char) -> i64 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => unreachable!(),
    }
}

fn to_decimal(snafu: &str) -> i64 {
    snafu.chars().rev().enumerate().fold(0, |acc, (i, c)| {
        if i == 0 {
            let value = parse_token(c);
            acc + value
        } else {
            let value = parse_token(c);
            acc + (5_i64.pow(i as u32) * value)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn test_part_one() {
        assert_eq!("2=-1=0", part_one(INPUT));
    }
}
