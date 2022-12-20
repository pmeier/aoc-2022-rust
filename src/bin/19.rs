use regex::Regex;
use std::{collections::HashSet, hash::Hash};

struct Blueprint {
    id: usize,
    ore_robot_cost: usize,
    clay_robot_cost: usize,
    obsidian_robot_cost: (usize, usize),
    geode_robot_cost: (usize, usize),
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct State {
    time: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
}

impl Blueprint {
    fn from_str(input: &str) -> Blueprint {
        let pattern = Regex::new(r"\d+").unwrap();
        let values: Vec<usize> = pattern
            .find_iter(input)
            .map(|m| m.as_str().parse::<usize>().unwrap())
            .collect();

        let id = values[0];
        let ore_robot_cost = values[1];
        let clay_robot_cost = values[2];
        let obsidian_robot_cost = (values[3], values[4]);
        let geode_robot_cost = (values[5], values[6]);

        Blueprint {
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        }
    }

    fn simulate(&self, minutes: usize, always_build: bool) -> usize {
        let max_ore_robots = [
            self.ore_robot_cost,
            self.clay_robot_cost,
            self.obsidian_robot_cost.0,
            self.geode_robot_cost.0,
        ]
        .into_iter()
        .max()
        .unwrap();
        let max_clay_robots = self.obsidian_robot_cost.1;
        let max_obsidian_robots = self.geode_robot_cost.1;

        let mut states: Vec<State> = vec![State {
            time: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }];
        let mut seen: HashSet<State> = HashSet::new();
        let mut max_geode = 0;

        while !states.is_empty() {
            let state = states.pop().unwrap();

            if state.time == minutes {
                max_geode = max_geode.max(state.geode);
                continue;
            }

            if !seen.insert(state) {
                continue;
            }

            let time = state.time + 1;
            let ore = state.ore + state.ore_robots;
            let clay = state.clay + state.clay_robots;
            let obsidian = state.obsidian + state.obsidian_robots;
            let geode = state.geode + state.geode_robots;

            if state.ore >= self.geode_robot_cost.0 && state.obsidian >= self.geode_robot_cost.1 {
                states.push(State {
                    time,
                    ore: ore - self.geode_robot_cost.0,
                    clay,
                    obsidian: obsidian - self.geode_robot_cost.1,
                    geode,
                    geode_robots: state.geode_robots + 1,
                    ..state
                });
                continue;
            }

            let mut build = false;

            if state.ore_robots < max_ore_robots && state.ore >= self.ore_robot_cost {
                build = true;
                states.push(State {
                    time,
                    ore: ore - self.ore_robot_cost,
                    clay,
                    obsidian,
                    geode,
                    ore_robots: state.ore_robots + 1,
                    ..state
                })
            }

            if state.clay_robots < max_clay_robots && state.ore >= self.clay_robot_cost {
                build = true;
                states.push(State {
                    time,
                    ore: ore - self.clay_robot_cost,
                    clay,
                    obsidian,
                    geode,
                    clay_robots: state.clay_robots + 1,
                    ..state
                })
            }

            if state.obsidian_robots < max_obsidian_robots
                && state.ore >= self.obsidian_robot_cost.0
                && state.clay >= self.obsidian_robot_cost.1
            {
                build = true;
                states.push(State {
                    time,
                    ore: ore - self.obsidian_robot_cost.0,
                    clay: clay - self.obsidian_robot_cost.1,
                    obsidian,
                    geode,
                    obsidian_robots: state.obsidian_robots + 1,
                    ..state
                })
            }

            if !(always_build && build) {
                states.push(State {
                    time,
                    ore,
                    clay,
                    obsidian,
                    geode,
                    ..state
                });
            }
        }

        max_geode
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let blueprint = Blueprint::from_str(line);
                (blueprint.id * blueprint.simulate(24, false)) as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .take(3)
            .map(|line| Blueprint::from_str(line).simulate(32, true) as u32)
            .product(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }
    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(3472));
    }
}
