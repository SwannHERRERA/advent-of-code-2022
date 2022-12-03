use std::fs;

use crate::{
    part1::{find_common_items, sums_items_priority},
    part2::{compute_summarize_of_priority, split_by_group_of_elves},
};

mod common;
mod part1;
mod part2;

fn main() {
    let input = fs::read_to_string("03-three/message.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let common_letters = find_common_items(input);
    let letters_value = sums_items_priority(common_letters);
    println!("{}", letters_value);
}

fn part2(input: &str) {
    let elves = split_by_group_of_elves(input);
    let sum = compute_summarize_of_priority(elves);
    println!("{sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        const EXPECTED_OUTPUT: u32 = 157;
        let items = find_common_items(INPUT);
        let sum_of_items_value = sums_items_priority(items);
        assert_eq!(EXPECTED_OUTPUT, sum_of_items_value);
    }

    #[test]
    fn test_part2() {
        const EXPECTED_RESULT: u32 = 70;

        let elves = split_by_group_of_elves(INPUT);
        let sum = compute_summarize_of_priority(elves);
        assert_eq!(EXPECTED_RESULT, sum);
    }
}
