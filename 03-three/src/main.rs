use itertools::multizip;
use std::{collections::HashSet, fs, thread};

fn main() {
    let input = fs::read_to_string("03-three/data.txt").unwrap();
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
    let sum = compute_summurize_of_priority(elves);
    println!("{sum}");
}

fn split_by_group_of_elves(input: &str) -> (Vec<String>, Vec<String>, Vec<String>) {
    let elves_1: Vec<String> = input.lines().skip(0).step_by(3).map(String::from).collect();
    let elves_2: Vec<String> = input.lines().skip(1).step_by(3).map(String::from).collect();
    let elves_3: Vec<String> = input.lines().skip(2).step_by(3).map(String::from).collect();

    (elves_1, elves_2, elves_3)
}

fn compute_summurize_of_priority(elves: (Vec<String>, Vec<String>, Vec<String>)) -> u32 {
    let mut threads = Vec::with_capacity(elves.0.len());

    for (elve_1, elve_2, elve_3) in multizip((elves.0, elves.1, elves.2)) {
        let thread = thread::spawn(move || -> u32 {
            find_priority_of_the_common_item(elve_1.as_str(), elve_2.as_str(), elve_3.as_str())
        });
        threads.push(thread);
    }
    threads
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .sum()
}

fn find_common_items(input: &str) -> Vec<char> {
    let mut common_items: Vec<char> = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        let (part1, part2) = split_at_center(line);
        let set = get_hashset_of_str(part1);
        let common_item: char = part2.chars().filter(|c| set.contains(c)).last().unwrap();
        common_items.push(common_item);
    }
    common_items
}

fn find_priority_of_the_common_item(elve_1: &str, elve_2: &str, elve_3: &str) -> u32 {
    let set_1 = get_hashset_of_str(elve_1);
    let set_2 = get_hashset_of_str(elve_2);
    let set_3 = get_hashset_of_str(elve_3);

    let intersection_between_1_and_2: HashSet<char> = set_1.intersection(&set_2).copied().collect();
    let mut intersection = intersection_between_1_and_2.intersection(&set_3);

    calculate_letter_priority(intersection.next().unwrap())
}

fn split_at_center(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn get_hashset_of_str(subject: &str) -> HashSet<char> {
    let mut set: HashSet<char> = HashSet::new();
    for item in subject.chars() {
        set.insert(item);
    }
    set
}

fn sums_items_priority(letters: Vec<char>) -> u32 {
    letters.iter().map(calculate_letter_priority).sum()
}

fn calculate_letter_priority(letter: &char) -> u32 {
    const UPPERCASE_A: u8 = b'A';
    const LOWERCASE_A: u8 = b'a';
    const UPPERCASE_OFFSET: u8 = 26;

    let ascii_value = *letter as u8;
    let priority: u32 = match letter {
        'a'..='z' => (ascii_value - LOWERCASE_A + 1).into(),
        'A'..='Z' => (ascii_value - UPPERCASE_A + 1 + UPPERCASE_OFFSET).into(),
        _ => unreachable!(),
    };
    priority
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::vec_eq;

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
    fn test_find_common_letter() {
        let expected = vec!['p', 'L', 'P', 'v', 't', 's'];
        let items = find_common_items(INPUT);
        println!("{:?} {:?}", items, expected);
        assert!(vec_eq(items, expected));
    }

    #[test]
    fn test_split_at_center() {
        const INPUT: &str = "PmmdzqPrVvPwwTWBwg";
        const EXPECTED_OUTPUT: (&str, &str) = ("PmmdzqPrV", "vPwwTWBwg");
        let result = split_at_center(INPUT);
        assert_eq!(EXPECTED_OUTPUT, result);
    }

    #[test]
    fn test_get_hashset_of_part1() {
        const PART_1: &str = "vPwwTWBwg";
        let set = get_hashset_of_str(PART_1);
        assert!(set.contains(&'v'));
        assert!(set.contains(&'P'));
        assert!(set.contains(&'w'));
        assert!(set.contains(&'T'));
        assert!(set.contains(&'W'));
        assert!(set.contains(&'B'));
        assert!(set.contains(&'g'));
        assert_eq!(set.contains(&'y'), false);
        assert_eq!(set.len(), 7);
    }

    #[test]
    fn test_part2() {
        const EXPECTED_RESULT: u32 = 70;

        let elves = split_by_group_of_elves(INPUT);
        let sum = compute_summurize_of_priority(elves);
        assert_eq!(EXPECTED_RESULT, sum);
    }
}
