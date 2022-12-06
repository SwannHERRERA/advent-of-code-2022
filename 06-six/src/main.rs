mod error;
mod prelude;

use std::fs;

use crate::prelude::*;

fn main() -> Result<()> {
    let input = fs::read_to_string("06-six/input.txt")?;
    let res_one = part_one(&input);
    println!("part one : {res_one}");
    let res_two = part_two(&input);
    println!("part two : {res_two}");
    Ok(())
}

fn part_one(input: &str) -> usize {
    const MARKER_SIZE: usize = 4;
    find_marker(input, MARKER_SIZE)
}

fn part_two(input: &str) -> usize {
    const MARKER_SIZE: usize = 14;
    find_marker(input, MARKER_SIZE)
}

fn find_marker(input: &str, marker_size: usize) -> usize {
    let len = input.len();
    let mut i = 0;
    while i < len {
        let slice = input[i..(i + marker_size)].to_string();
        let index_of_jump = find_same_char(slice);
        if index_of_jump == 0 {
            return i + marker_size;
        }
        i += index_of_jump as usize;
    }
    0
}

fn find_same_char(slice: String) -> i32 {
    let mut copy = slice.clone();
    for (idx, c) in copy.drain(..).enumerate() {
        if slice[(idx + 1)..].contains(c) {
            return (idx + 1) as i32;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::vec_eq;

    #[test]
    fn test_part_one() {
        let inputs: Vec<&str> = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];

        let exptected_result: Vec<usize> = vec![7, 5, 6, 10, 11];
        let res: Vec<usize> = inputs.iter().map(|input| part_one(input)).collect();
        assert!(vec_eq(exptected_result, res));
    }

    #[test]
    fn test_part_two() {
        let inputs: Vec<&str> = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];

        let exptected_result: Vec<usize> = vec![19, 23, 23, 29, 26];

        let res: Vec<usize> = inputs.iter().map(|input| part_two(input)).collect();
        println!("{:?}", res);
        assert!(vec_eq(exptected_result, res));
    }
}
