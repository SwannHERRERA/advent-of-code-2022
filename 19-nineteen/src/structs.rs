use std::ops::{Add, Mul};

use parse_display::{Display, FromStr};

#[derive(Debug, Display, FromStr, PartialEq)]
#[display("Blueprint {id}: Each ore robot costs {ore_robot_cost} ore. Each clay robot costs {clay_robot_cost} ore. Each obsidian robot costs {obisidian_robot_cost_in_ore} ore and {obisidian_robot_cost_in_clay} clay. Each geode robot costs {geode_robot_ore_cost} ore and {geode_robot_obisidan_cost} obsidian.")]
pub struct BlueprintFromInput {
    id: usize,
    ore_robot_cost: usize,
    clay_robot_cost: usize,
    obisidian_robot_cost_in_ore: usize,
    obisidian_robot_cost_in_clay: usize,
    geode_robot_ore_cost: usize,
    geode_robot_obisidan_cost: usize,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Resources {
    pub ore: usize,
    pub clay: usize,
    pub obsidian: usize,
}

impl Resources {
    pub fn checked_sub(self, other: Self) -> Option<Resources> {
        Some(Resources {
            ore: self.ore.checked_sub(other.ore)?,
            clay: self.clay.checked_sub(other.clay)?,
            obsidian: self.obsidian.checked_sub(other.obsidian)?,
        })
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
        }
    }
}

impl Mul<usize> for Resources {
    type Output = Self;

    fn mul(self, other: usize) -> Self {
        Self {
            ore: self.ore * other,
            clay: self.clay * other,
            obsidian: self.obsidian * other,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Blueprint {
    pub id: usize,
    pub ore_robot_cost: Resources,
    pub clay_robot_cost: Resources,
    pub obsidian_robot_cost: Resources,
    pub geode_robot_cost: Resources,
    pub max_ore_cost: usize,
}

impl From<&BlueprintFromInput> for Blueprint {
    fn from(candidate: &BlueprintFromInput) -> Self {
        let ore_costs = [
            candidate.geode_robot_ore_cost,
            candidate.obisidian_robot_cost_in_ore,
            candidate.clay_robot_cost,
            candidate.ore_robot_cost,
        ];
        let max_ore_cost = *ore_costs.iter().max().unwrap();

        Blueprint {
            id: candidate.id,
            ore_robot_cost: Resources {
                ore: candidate.ore_robot_cost,
                clay: 0,
                obsidian: 0,
            },
            clay_robot_cost: Resources {
                ore: candidate.clay_robot_cost,
                clay: 0,
                obsidian: 0,
            },
            obsidian_robot_cost: Resources {
                ore: candidate.obisidian_robot_cost_in_ore,
                clay: candidate.obisidian_robot_cost_in_clay,
                obsidian: 0,
            },
            geode_robot_cost: Resources {
                ore: candidate.geode_robot_ore_cost,
                clay: 0,
                obsidian: candidate.geode_robot_obisidan_cost,
            },
            max_ore_cost,
        }
    }
}
