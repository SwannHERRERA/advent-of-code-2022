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
        let mut max_geode = 0;
        branch_and_bound(&blueprint, Game::new(24), &mut max_geode);
        blueprint.id as usize * max_geode as usize
    }).sum()
}

fn part_two(input: &str) -> usize {
    let blueprints = parse_input(input);

    blueprints.iter().take(3).map(Blueprint::from).map(|blueprint| {
        let mut solution = 0;
        branch_and_bound(&blueprint, Game::new(32), &mut solution);
        solution as usize
    }).product()
}

fn parse_input(input: &str) -> Vec<BlueprintFromInput> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn branch_and_bound(blueprint: &Blueprint, state: Game, max: &mut usize) {
    *max = state.geodes.max(*max);
    for state in state.branch(blueprint) {
        if state.bound(blueprint.geode_robot_cost.obsidian) > *max {
            branch_and_bound(blueprint, state, max);
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
}
