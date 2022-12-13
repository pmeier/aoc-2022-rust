extern crate pathfinding;

use std::ops::Deref;

use pathfinding::prelude::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    position: (usize, usize),
    elevation: usize,
}

impl Node {
    fn from_char(position: (usize, usize), c: char, inline_start: bool) -> Node {
        Node {
            position,
            elevation: match c {
                'S' => usize::from(inline_start),
                'E' => 27,
                ascii_lowercase => ascii_lowercase as usize - 'a' as usize + 1,
            },
        }
    }

    fn successors(&self, grid: &Grid) -> Vec<(Node, usize)> {
        let mut successors = Vec::new();
        let (x, y) = self.position;
        let max_successor_elevation = self.elevation + 1;

        if x > 0 && grid[y][x - 1].elevation <= max_successor_elevation {
            successors.push(grid[y][x - 1].clone());
        }
        if x < grid[0].len() - 1 && grid[y][x + 1].elevation <= max_successor_elevation {
            successors.push(grid[y][x + 1].clone());
        }
        if y > 0 && grid[y - 1][x].elevation <= max_successor_elevation {
            successors.push(grid[y - 1][x].clone());
        }
        if y < grid.len() - 1 && grid[y + 1][x].elevation <= max_successor_elevation {
            successors.push(grid[y + 1][x].clone());
        }

        successors
            .into_iter()
            .map(|node| (node, 1))
            .collect::<Vec<(Node, usize)>>()
    }

    fn distance(&self, other: &Node) -> usize {
        self.position.0.abs_diff(other.position.0) + self.position.1.abs_diff(other.position.1)
    }
}

struct Grid {
    values: Vec<Vec<Node>>,
}

impl Grid {
    fn from_str(input: &str, inline_start: bool) -> Grid {
        Grid {
            values: input
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, c)| Node::from_char((x, y), c, inline_start))
                        .collect()
                })
                .collect(),
        }
    }

    fn find_nodes(&self, elevation: usize) -> Vec<&Node> {
        self.iter()
            .flatten()
            .filter(|node| node.elevation == elevation)
            .collect()
    }
}

impl Deref for Grid {
    type Target = Vec<Vec<Node>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

fn solve(grid: &Grid, start: &Node, goal: &Node) -> Option<u32> {
    astar(
        start,
        |node| node.successors(grid),
        |node| node.distance(goal),
        |node| node == goal,
    )
    .map(|(_, num_steps)| num_steps as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input, false);

    let start: &Node = grid.find_nodes(0).first().unwrap();
    let goal: &Node = grid.find_nodes(27).first().unwrap();

    Some(solve(&grid, start, goal).unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input, true);

    let mut starts: Vec<&Node> = grid.find_nodes(1);
    let goal: &Node = grid.find_nodes(27).first().unwrap();

    starts.sort_by_key(|node| node.distance(goal));

    let mut lens = Vec::new();
    while !starts.is_empty() {
        let start = starts.remove(0);
        if let Some(len) = solve(&grid, start, goal) {
            lens.push(len);
            starts.retain(|node| node.distance(goal) < len as usize);
        }
    }
    Some(lens.into_iter().min().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
