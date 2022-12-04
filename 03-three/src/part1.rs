use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::common::{calculate_letter_priority, get_hashset_of_str};

pub fn find_common_items(input: &str) -> Vec<char> {
    let lines_count = input.lines().count();
    let common_items: Arc<Mutex<Vec<char>>> = Arc::new(Mutex::new(Vec::with_capacity(lines_count)));
    let mut threads = Vec::with_capacity(lines_count);
    for line in input.lines() {
        let common_items = common_items.clone();
        let line = line.to_owned();
        threads.push(thread::spawn(move || {
            let (part1, part2) = split_at_center(line.as_str());
            let set = get_hashset_of_str(part1);
            let common_item: char = part2.chars().filter(|c| set.contains(c)).last().unwrap();
            common_items.lock().unwrap().push(common_item);
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    let result = common_items.lock().unwrap().to_vec();
    result
}

pub fn split_at_center(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

pub fn sums_items_priority(letters: Vec<char>) -> u32 {
    letters.iter().map(calculate_letter_priority).sum()
}

#[allow(unused)]
pub fn find_common_items_single_thread(input: &str) -> Vec<char> {
    let mut common_items: Vec<char> = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        let (part1, part2) = split_at_center(line);
        let set = get_hashset_of_str(part1);
        let common_item: char = part2.chars().filter(|c| set.contains(c)).last().unwrap();
        common_items.push(common_item);
    }
    common_items
}

#[cfg(test)]
mod tests {
    use test_utils::vec_eq;

    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_split_at_center() {
        const INPUT: &str = "PmmdzqPrVvPwwTWBwg";
        const EXPECTED_OUTPUT: (&str, &str) = ("PmmdzqPrV", "vPwwTWBwg");
        let result = split_at_center(INPUT);
        assert_eq!(EXPECTED_OUTPUT, result);
    }

    #[test]
    fn test_find_common_items() {
        let expected = vec!['p', 'L', 'P', 'v', 't', 's'];
        let items = find_common_items(INPUT);
        assert_eq!(expected.len(), items.len());

        for item in expected {
            let is_find = items.iter().find(|element| **element == item);
            assert!(is_find.is_some());
        }
    }

    #[test]
    fn test_find_common_items_single_thread() {
        let expected = vec!['p', 'L', 'P', 'v', 't', 's'];
        let items = find_common_items_single_thread(INPUT);
        println!("{:?} {:?}", items, expected);
        assert!(vec_eq(items, expected));
    }
}
