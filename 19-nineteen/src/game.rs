use crate::structs::{Blueprint, Resources};

#[derive(Debug, Clone, Copy)]
pub struct Game {
    pub minutes_remaining: usize,
    pub geodes: usize,
    pub resources: Resources,
    pub resources_production: Resources,
}

impl Game {
    pub fn new(minutes_remaining: usize) -> Self {
        Self {
            minutes_remaining,
            geodes: 0,
            resources: Default::default(),
            resources_production: Resources {
                ore: 1,
                clay: 0,
                obsidian: 0,
            },
        }
    }

    pub fn branch(self, blueprint: &Blueprint) -> impl Iterator<Item = Self> + '_ {
        const MAX_BRANCH_POSSIBILITY: usize = 4;
        let mut branches = Vec::with_capacity(MAX_BRANCH_POSSIBILITY);

        let ore_robot_viable = self.resources_production.ore < blueprint.max_ore_cost;
        let clay_robot_viable = self.resources_production.clay < blueprint.obsidian_robot_cost.clay;
        let obsidian_robot_viable = self.resources_production.obsidian
            < blueprint.geode_robot_cost.obsidian
            && self.resources_production.clay > 0;
        let geode_robot_viable = self.resources_production.obsidian > 0;

        if ore_robot_viable {
            branches.push(self.chose_robot(
                blueprint.ore_robot_cost,
                Resources {
                    ore: 1,
                    clay: 0,
                    obsidian: 0,
                },
            ));
        }
        if clay_robot_viable {
            branches.push(self.chose_robot(
                blueprint.clay_robot_cost,
                Resources {
                    ore: 0,
                    clay: 1,
                    obsidian: 0,
                },
            ));
        }
        if obsidian_robot_viable {
            branches.push(self.chose_robot(
                blueprint.obsidian_robot_cost,
                Resources {
                    ore: 0,
                    clay: 0,
                    obsidian: 1,
                },
            ));
        }
        if geode_robot_viable {
            let new_game = self.chose_robot(
                blueprint.geode_robot_cost,
                Resources {
                    ore: 1,
                    clay: 0,
                    obsidian: 0,
                },
            );
            let new_game = new_game.map(|game| Game {
                geodes: game.geodes + game.minutes_remaining,
                ..game
            });
            branches.push(new_game);
        }

        branches.into_iter().flatten()
    }

    pub fn bound(self, obisidian_cost: usize) -> usize {
        let (_, _, geodes) = (0..self.minutes_remaining).rev().fold(
            (
                self.resources.obsidian,
                self.resources_production.obsidian,
                self.geodes,
            ),
            |(obsidian, rate, geodes), minutes_remaining| {
                if obsidian >= obisidian_cost {
                    (
                        obsidian + rate - obisidian_cost,
                        rate,
                        geodes.saturating_add(minutes_remaining),
                    )
                } else {
                    (obsidian + rate, rate + 1, geodes)
                }
            },
        );
        geodes
    }

    fn chose_robot(self, cost: Resources, robot: Resources) -> Option<Game> {
        (1..self.minutes_remaining).rev().zip(0..).find_map(
            |(minutes_remaining, minutes_passed)| {
                let resources = self.resources + self.resources_production * minutes_passed;
                resources.checked_sub(cost).map(|resources| Game {
                    minutes_remaining,
                    resources: resources + self.resources_production,
                    resources_production: self.resources_production + robot,
                    ..self
                })
            },
        )
    }
}
