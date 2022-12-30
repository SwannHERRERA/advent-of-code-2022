use std::fs;

use game::Game;
use structs::{Blueprint, BlueprintFromInput};

mod structs;
mod game;


fn main() {
    let input = fs::read_to_string("19-nineteen/input.txt").unwrap();
    let part_one = part_one(&input);
    println!("part one : {}", part_one);
    let part_two = part_two(&input);
    println!("part two : {}", part_two);
}

fn part_one(input: &str) -> usize {
    let blueprints = parse_input(input);

    blueprints.iter().map(Blueprint::from).map(|blueprint| {
        let mut best = 0;
        branch_and_bound(&blueprint, Game::new(24), &mut best);
        blueprint.id as usize * best as usize
    }).sum()
}

fn part_two(input: &str) -> usize {
    let blueprints = parse_input(input);

    blueprints.iter().take(3).map(Blueprint::from).map(|blueprint| {
        let mut best = 0;
        branch_and_bound(&blueprint, Game::new(32), &mut best);
        best as usize
    }).product()
}

fn parse_input(input: &str) -> Vec<BlueprintFromInput> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn branch_and_bound(blueprint: &Blueprint, state: Game, best: &mut usize) {
    *best = state.geodes.max(*best);
    for state in state.branch(blueprint) {
        if state.bound(blueprint) > *best {
            branch_and_bound(blueprint, state, best);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_part_one() {
        let res = part_one(INPUT);
        assert_eq!(33, res);
    }

    #[test]
    fn test_part_two() {
        let res = part_two(INPUT);
        assert_eq!(56 * 62, res);
    }
}
