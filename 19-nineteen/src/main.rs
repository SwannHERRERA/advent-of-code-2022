use std::{rc::Rc, cell::RefCell, fs, collections::HashSet};
use rayon::prelude::*;

use parse_display::{Display, FromStr};

fn main() {
    let input = fs::read_to_string("19-nineteen/input.txt").unwrap();
    let part_one = part_one(&input);
    println!("part one : {}", part_one);
}

#[derive(Debug, Display, FromStr, PartialEq)]
#[display("Blueprint {id}: Each ore robot costs {ore_robot_cost} ore. Each clay robot costs {clay_robot_cost} ore. Each obsidian robot costs {obisidian_robot_cost_in_ore} ore and {obisidian_robot_cost_in_clay} clay. Each geode robot costs {geode_robot_ore_cost} ore and {geode_robot_obisidan_cost} obsidian.")]
struct Blueprint {
    id: usize,
    ore_robot_cost: usize,
    clay_robot_cost: usize,
    obisidian_robot_cost_in_ore: usize,
    obisidian_robot_cost_in_clay: usize,
    geode_robot_ore_cost: usize,
    geode_robot_obisidan_cost: usize,
}

impl Blueprint {
    fn new(
        id: usize,
        ore_robot_cost: usize,
        clay_robot_cost: usize,
        obisidian_robot_cost_in_ore: usize,
        obisidian_robot_cost_in_clay: usize,
        geode_robot_ore_cost: usize,
        geode_robot_obisidan_cost: usize,
    ) -> Self {
        Blueprint {
            id,
            ore_robot_cost,
            clay_robot_cost,
            obisidian_robot_cost_in_ore,
            obisidian_robot_cost_in_clay,
            geode_robot_ore_cost,
            geode_robot_obisidan_cost,
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
    minutes_remaining: usize,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            minutes_remaining: 24,
        }
    }
}

impl Game {
    fn construct_geode_robot(&self, blueprint: &Blueprint) -> Self {
        let mut new_game = self.clone();
        new_game.ore -= blueprint.geode_robot_ore_cost;
        new_game.obsidian -= blueprint.geode_robot_obisidan_cost;
        new_game = new_game.step();
        new_game.geode_robots += 1;
        new_game
    }

    fn construct_obisidian_robot(&self, blueprint: &Blueprint) -> Self {
        let mut new_game = self.clone();
        new_game.ore -= blueprint.obisidian_robot_cost_in_ore;
        new_game.clay -= blueprint.obisidian_robot_cost_in_clay;
        new_game = new_game.step();
        new_game.obsidian_robots += 1;
        new_game
    }
    
    fn construct_clay_robot(&self, blueprint: &Blueprint) -> Self {
        let mut new_game = self.clone();
        new_game.ore -= blueprint.clay_robot_cost;
        new_game = new_game.step();
        new_game.clay_robots += 1;
        new_game
    }
    
    fn construct_ore_robot(&self, blueprint: &Blueprint) -> Self {
        let mut new_game = self.clone();
        new_game.ore -= blueprint.ore_robot_cost;
        new_game = new_game.step();
        new_game.ore_robots += 1;
        new_game
    }

    fn step(&self) -> Self {
        let mut game = self.clone();
        game.ore += game.ore_robots;
        game.clay += game.clay_robots;
        game.obsidian += game.obsidian_robots;
        game.geode += game.geode_robots;
        game.minutes_remaining -= 1;
        game
    }
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn part_one(input: &str) -> usize {
    let blueprints = parse_input(input);
    blueprints.par_iter().map(|blueprint| {
        println!("{blueprint}");
        let game = Game::default();
        let geodes: Rc<RefCell<HashSet<usize>>> = Rc::new(RefCell::new(HashSet::new()));
        run_game(game, blueprint, geodes.clone());
        let geodes = geodes.take();
        println!("{:?}, max: {}", geodes, geodes.iter().max().unwrap());
        geodes.iter().max().copied().unwrap() * blueprint.id
    }).sum()
}

fn run_game(game: Game, blueprint: &Blueprint, results: Rc<RefCell<HashSet<usize>>>) {
    if game.minutes_remaining == 0 {
        let mut vec = results.borrow_mut();
        vec.insert(game.geode);
        return;
    }
    run_game(game.step(), blueprint, results.clone());
    let costs_in_ore = [blueprint.clay_robot_cost, blueprint.obisidian_robot_cost_in_ore, blueprint.geode_robot_ore_cost];
    let max_cost = costs_in_ore.iter().max().unwrap();
    if *max_cost < game.ore_robots && game.ore >= blueprint.ore_robot_cost {
        let ore = game.construct_ore_robot(blueprint);
        run_game(ore, blueprint, results.clone());
    }
    if game.ore >= blueprint.geode_robot_ore_cost && game.obsidian >= blueprint.geode_robot_obisidan_cost {
        let geode = game.construct_geode_robot(blueprint);
        run_game(geode, blueprint, results);
        return;
    }
    if game.obsidian_robots < blueprint.geode_robot_obisidan_cost
        && game.ore >= blueprint.obisidian_robot_cost_in_ore
        && game.clay >= blueprint.obisidian_robot_cost_in_clay {
        let obsidian = game.construct_obisidian_robot(blueprint);
        run_game(obsidian, blueprint, results);
        return;
    }
    if game.clay_robots < blueprint.obisidian_robot_cost_in_clay && game.ore >= blueprint.clay_robot_cost {
        let clay = game.construct_clay_robot(blueprint);
        run_game(clay, blueprint, results);
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
