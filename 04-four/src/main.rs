use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("04-four/input.txt").unwrap();
    let res_part_one = part_one(&input);
    let res_part_two = part_two(&input);
    println!("part one : {}", res_part_one);
    println!("part two : {}", res_part_two);
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter(|line| is_one_assigment_fully_contain_the_other(line))
        .count()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .filter(|line| is_one_assigment_overlap_the_other(line))
        .count()
}

fn is_one_assigment_overlap_the_other(line: &str) -> bool {
    let (first, second): (&str, &str) = line.split_once(',').unwrap();
    let first_assignments = get_range_from_input(first);
    let second_assignments = get_range_from_input(second);

    !first_assignments.is_disjoint(&second_assignments)
}

fn is_one_assigment_fully_contain_the_other(line: &str) -> bool {
    let (first, second): (&str, &str) = line.split_once(',').unwrap();
    let first_assignments = get_range_from_input(first);
    let second_assignments = get_range_from_input(second);

    first_assignments.is_subset(&second_assignments)
        || first_assignments.is_superset(&second_assignments)
}

fn get_range_from_input(input: &str) -> HashSet<usize> {
    let numbers: (&str, &str) = input.split_once('-').unwrap();
    let first_born: usize = numbers.0.parse().unwrap();
    let second_born: usize = numbers.1.parse().unwrap();

    (first_born..=second_born).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part_one() {
        const EXPECTED_OUTPUT: usize = 2;
        let res = part_one(INPUT);
        assert_eq!(EXPECTED_OUTPUT, res);
    }

    #[test]
    fn test_part_two() {
        const EXPECTED_OUTPUT: usize = 4;
        let res = part_two(INPUT);
        assert_eq!(EXPECTED_OUTPUT, res);
    }
}
