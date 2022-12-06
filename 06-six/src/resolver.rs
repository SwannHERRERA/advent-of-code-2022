use std::collections::VecDeque;

use crate::utils::find_marker;

pub fn part_one(input: &str) -> usize {
    const MARKER_SIZE: usize = 4;
    find_marker(input, MARKER_SIZE)
}

pub fn part_two(input: &str) -> usize {
    const MARKER_SIZE: usize = 14;
    find_marker(input, MARKER_SIZE)
}

#[allow(unused)]
pub fn start_one(input: &str, marker_size: usize) {
    let input: Vec<char> = input
        .chars()
        .collect();
    let marker_size = marker_size - 1;
    let mut marker: VecDeque<char> = VecDeque::new();
    for i in 0..marker_size {
        marker.push_back(*input.get(i).unwrap());
    }
    let mut counter = marker_size.clone();
    while counter < input.len() {
        let new_char: char = *input.get(counter).unwrap();
        while marker.contains(&new_char) {
            marker.pop_front();
            counter += 1;
        }
        if marker.len() == marker_size {
            println!("{}", counter + 1);
            return;
        }
        while marker.len() != marker_size {
            if marker.contains(&input.get(counter - (marker_size - marker.len())).unwrap()) {
                marker.pop_front();
                counter += 1
            } else {
                marker.push_back(*input.get(counter - (marker_size - marker.len())).unwrap());
            }
        }
    }
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
