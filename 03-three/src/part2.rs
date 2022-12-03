use itertools::multizip;
use std::{collections::HashSet, thread};

use crate::common::{calculate_letter_priority, get_hashset_of_str};

pub fn split_by_group_of_elves(input: &str) -> (Vec<String>, Vec<String>, Vec<String>) {
    let elves_1: Vec<String> = input.lines().skip(0).step_by(3).map(String::from).collect();
    let elves_2: Vec<String> = input.lines().skip(1).step_by(3).map(String::from).collect();
    let elves_3: Vec<String> = input.lines().skip(2).step_by(3).map(String::from).collect();

    (elves_1, elves_2, elves_3)
}

pub fn compute_summurize_of_priority(elves: (Vec<String>, Vec<String>, Vec<String>)) -> u32 {
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

fn find_priority_of_the_common_item(elve_1: &str, elve_2: &str, elve_3: &str) -> u32 {
    let set_1 = get_hashset_of_str(elve_1);
    let set_2 = get_hashset_of_str(elve_2);
    let set_3 = get_hashset_of_str(elve_3);

    let intersection_between_1_and_2: HashSet<char> = set_1.intersection(&set_2).copied().collect();
    let mut intersection = intersection_between_1_and_2.intersection(&set_3);

    calculate_letter_priority(intersection.next().unwrap())
}
