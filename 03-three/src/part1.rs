use crate::common::{calculate_letter_priority, get_hashset_of_str};


pub fn find_common_items(input: &str) -> Vec<char> {
    let mut common_items: Vec<char> = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        let (part1, part2) = split_at_center(line);
        let set = get_hashset_of_str(part1);
        let common_item: char = part2.chars().filter(|c| set.contains(c)).last().unwrap();
        common_items.push(common_item);
    }
    common_items
}

pub fn split_at_center(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

pub fn sums_items_priority(letters: Vec<char>) -> u32 {
    letters.iter().map(calculate_letter_priority).sum()
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
    fn test_find_common_letter() {
        let expected = vec!['p', 'L', 'P', 'v', 't', 's'];
        let items = find_common_items(INPUT);
        println!("{:?} {:?}", items, expected);
        assert!(vec_eq(items, expected));
    }
}
