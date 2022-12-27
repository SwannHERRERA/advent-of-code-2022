use std::{rc::Rc, cell::RefCell, fs, collections::{HashSet, VecDeque}, sync::{Mutex, Arc}, thread};
use rayon::prelude::*;

use parse_display::{Display, FromStr};

fn main() {
    let input = fs::read_to_string("19-nineteen/input.txt").unwrap();
    let part_one = part_one(&input);
    println!("part one : {}", part_one);
}

#[derive(Debug, Display, FromStr, PartialEq, Clone)]
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
    let vecdeq: Arc<Mutex<VecDeque<(Blueprint, Game, usize, usize)>>> = Arc::new(Mutex::new(VecDeque::with_capacity(200)));
    let geode_count: Arc<Mutex<HashSet<(usize, usize)>>> = Arc::new(Mutex::new(HashSet::new()));

    for blueprint in &blueprints {
        let game = Game::default();
        let costs_in_ore = [blueprint.clay_robot_cost, blueprint.obisidian_robot_cost_in_ore, blueprint.geode_robot_ore_cost];
        let max_cost = *costs_in_ore.iter().max().unwrap();
        push(vecdeq.clone(), blueprint.clone(), game, 24, max_cost);
        while let Some((blueprint, game, time, max_ore)) = pop_front(vecdeq.clone()) {
            if time == 0 {
                insert(geode_count.clone(), game.geode, blueprint.id);
                continue;
            }
            push(vecdeq.clone(), blueprint.clone(), game.step(), time - 1, max_ore);
            if max_ore < game.ore_robots && game.ore >= blueprint.ore_robot_cost {
                push(vecdeq.clone(), blueprint.clone(), game.step().construct_ore_robot(&blueprint), time - 1, max_ore);
            }
            if game.obsidian >= blueprint.geode_robot_obisidan_cost && game.ore >= blueprint.geode_robot_ore_cost {
                push(vecdeq.clone(), blueprint.clone(), game.step().construct_geode_robot(&blueprint), time - 1, max_ore);
                continue;
            }
            if game.obsidian_robots < blueprint.geode_robot_obisidan_cost
                && game.clay >= blueprint.obisidian_robot_cost_in_clay
                && game.ore >= blueprint.obisidian_robot_cost_in_ore {
                push(vecdeq.clone(), blueprint.clone(), game.step().construct_obisidian_robot(&blueprint), time - 1, max_ore);
                continue;
            }
            if game.clay_robots < blueprint.obisidian_robot_cost_in_clay && game.ore >= blueprint.clay_robot_cost {
                push(vecdeq.clone(), blueprint.clone(), game.step().construct_clay_robot(&blueprint), time - 1, max_ore);
            }
            if len(vecdeq.clone()) >= 20 {
                run_thread(vecdeq.clone(), geode_count.clone());
            }
        }
    }
    let geode = geode_count.lock().unwrap();
    blueprints.iter().map(|blueprint| {
        geode.iter().filter(|(_, id)| *id == blueprint.id).map(|(count, _id)| count).max().unwrap() * blueprint.id
    }).sum()
}

fn run_thread(vecdeq: Arc<Mutex<VecDeque<(Blueprint, Game, usize, usize)>>>, geode_count: Arc<Mutex<HashSet<(usize, usize)>>>) {
    let mut threads = Vec::with_capacity(20);
    for _ in 0..20 {
        let vecdeq = vecdeq.clone();
        let geode_count = geode_count.clone();
        threads.push(thread::spawn(move || {
            while let Some((blueprint, game, time, max_ore)) = pop_front(vecdeq.clone()) {
                if time == 0 {
                    insert(geode_count.clone(), game.geode, blueprint.id);
                    continue;
                }
                push(vecdeq.clone(), blueprint.clone(), game.step(), time - 1, max_ore);
                if max_ore < game.ore_robots && game.ore >= blueprint.ore_robot_cost {
                    push(vecdeq.clone(), blueprint.clone(), game.step().construct_ore_robot(&blueprint), time - 1, max_ore);
                }
                if game.obsidian >= blueprint.geode_robot_obisidan_cost && game.ore >= blueprint.geode_robot_ore_cost {
                    push(vecdeq.clone(), blueprint.clone(), game.step().construct_geode_robot(&blueprint), time - 1, max_ore);
                    continue;
                }
                if game.obsidian_robots < blueprint.geode_robot_obisidan_cost
                    && game.clay >= blueprint.obisidian_robot_cost_in_clay
                    && game.ore >= blueprint.obisidian_robot_cost_in_ore {
                    push(vecdeq.clone(), blueprint.clone(), game.step().construct_obisidian_robot(&blueprint), time - 1, max_ore);
                    continue;
                }
                if game.clay_robots < blueprint.obisidian_robot_cost_in_clay && game.ore >= blueprint.clay_robot_cost {
                    push(vecdeq.clone(), blueprint.clone(), game.step().construct_clay_robot(&blueprint), time - 1, max_ore);
                }
            }
        }));
    }
}

fn len(target: Arc<Mutex<VecDeque<(Blueprint, Game, usize, usize)>>>) -> usize {
    let target = target.lock().unwrap();
    target.len()
}

fn insert(target: Arc<Mutex<HashSet<(usize, usize)>>>, geode: usize, blueprint_id: usize) {
    let mut target = target.lock().unwrap();
    target.insert((geode, blueprint_id));
}

fn pop_front(target: Arc<Mutex<VecDeque<(Blueprint, Game, usize, usize)>>>) -> Option<(Blueprint, Game, usize, usize)> {
    let mut target = target.lock().unwrap();
    target.pop_front()
}

fn push(target: Arc<Mutex<VecDeque<(Blueprint, Game, usize, usize)>>>, blueprint: Blueprint, game: Game, time: usize, ore_max_cost: usize) {
    let mut target = target.lock().unwrap();
    target.push_back((blueprint.clone(), game, 24, ore_max_cost));
}

fn run_game(game: Game, blueprint: &Blueprint, results: Rc<RefCell<HashSet<usize>>>, minutes_remaining: usize, ore_max_cost: usize) {
    if minutes_remaining == 0 {
        let mut vec = results.borrow_mut();
        vec.insert(game.step().geode);
        return;
    }
    run_game(game.step(), blueprint, results.clone(), minutes_remaining - 1, ore_max_cost);
    if ore_max_cost < game.ore_robots && game.ore >= blueprint.ore_robot_cost {
        let ore = game.construct_ore_robot(blueprint);
        run_game(ore, blueprint, results.clone(), minutes_remaining - 1, ore_max_cost);
    }
    if  game.ore >= blueprint.geode_robot_ore_cost && game.obsidian >= blueprint.geode_robot_obisidan_cost {
        let geode = game.construct_geode_robot(blueprint);
        run_game(geode, blueprint, results.clone(), minutes_remaining - 1, ore_max_cost);
        return;
    }
    if game.obsidian_robots < blueprint.geode_robot_obisidan_cost
        && game.ore >= blueprint.obisidian_robot_cost_in_ore
        && game.clay >= blueprint.obisidian_robot_cost_in_clay {
        let obsidian = game.construct_obisidian_robot(blueprint);
        run_game(obsidian, blueprint, results.clone(), minutes_remaining - 1, ore_max_cost);
        return;
    }
    if game.clay_robots < blueprint.obisidian_robot_cost_in_clay && game.ore >= blueprint.clay_robot_cost {
        let clay = game.construct_clay_robot(blueprint);
        run_game(clay, blueprint, results, minutes_remaining - 1, ore_max_cost);
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
